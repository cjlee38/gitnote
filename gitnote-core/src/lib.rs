use std::path::Path;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jint;
use serde::Serialize;
use crate::diff::SimilarDiffer;
use crate::handlers::{NoteArgs, NoteHandler};
use crate::libgit::{Libgit, ManualLibgit};
use crate::path::{PathResolver, Paths};
use crate::repository::NoteRepository;

pub mod handlers;
pub mod libgit;
pub mod repository;
pub mod stdio;
pub mod note;
pub mod utils;
pub mod cli;
pub mod path;
pub mod diff;
pub mod config;

#[cfg(test)]
pub mod testlib;

#[repr(C)]
#[derive(Serialize)]
pub struct Response {
    exit_code: i32,
    text: String,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            exit_code: 0,
            text: "".to_string(),
        }
    }
}

pub struct NoteLibArgs {
    paths: Paths,
    line: Option<i32>,
    message: Option<String>,
}

impl NoteArgs for NoteLibArgs {
    fn paths(&self) -> &Paths {
        &self.paths
    }

    fn user_line(&self) -> usize {
        self.line.unwrap() as usize
    }

    fn sys_line(&self) -> usize {
        self.line.unwrap() as usize - 1
    }

    fn message(&self) -> String {
        self.message.as_ref().unwrap().clone()
    }
}

#[no_mangle]
pub extern "system" fn Java_io_cjlee_gitnote_core_JniCoreConnector_add0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    exec_path: JString<'local>,
    file_path: JString<'local>,
    line: jint,
    message: JString<'local>
) -> JString<'local> {
    let handler = note_handler();
    let paths = paths(&mut env, &exec_path, &file_path);

    let args = NoteLibArgs {
        paths,
        line: Some(line),
        message: Some(peel_string(&mut env, &message)),
    };

    handler.add_note(&args).unwrap();
    let response = Response::default();
    new_json_string(env, &response)
}

#[no_mangle]
pub extern "system" fn Java_io_cjlee_gitnote_core_JniCoreConnector_read0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    exec_path: JString<'local>,
    file_path: JString<'local>,
) -> JString<'local> {
    let handler = note_handler();
    let paths = paths(&mut env, &exec_path, &file_path);
    let args = NoteLibArgs {
        paths,
        line: None,
        message: None,
    };
    let ledger = handler.read_note(&args).unwrap();
    let note = ledger.opaque_note();
    let r = Response {
        exit_code: 0,
        text: serde_json::to_string(&note).unwrap(),
    };
    new_json_string(env, &r)
}

#[no_mangle]
pub extern "system" fn Java_io_cjlee_gitnote_core_JniCoreConnector_update0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    exec_path: JString<'local>,
    file_path: JString<'local>,
    line: jint,
    message: JString<'local>,
) -> JString<'local> {
    let handler = note_handler();
    let paths = paths(&mut env, &exec_path, &file_path);

    let args = NoteLibArgs {
        paths,
        line: Some(line),
        message: Some(peel_string(&mut env, &message)),
    };

    handler.edit_note(&args).unwrap();
    let response = Response::default();
    new_json_string(env, &response)
}

#[no_mangle]
pub extern "system" fn Java_io_cjlee_gitnote_core_JniCoreConnector_delete0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    exec_path: JString<'local>,
    file_path: JString<'local>,
    line: jint,
) -> JString<'local> {
    let handler = note_handler();
    let paths = paths(&mut env, &exec_path, &file_path);

    let args = NoteLibArgs {
        paths,
        line: Some(line),
        message: None,
    };
    handler.delete_note(&args).unwrap();
    let response = Response::default();
    new_json_string(env, &response)
}

fn note_handler() -> NoteHandler<ManualLibgit<SimilarDiffer>> {
    let libgit = ManualLibgit::new(SimilarDiffer);
    NoteHandler::new(NoteRepository::new(libgit))
}

fn paths(env: &mut JNIEnv, exec_path: &JString, file_path: &JString) -> Paths {
    let exec_path = peel_string(env, exec_path);
    let current_path = Path::new(&exec_path);
    let file_path = &peel_string(env, file_path);
    PathResolver::resolve(current_path, file_path).unwrap()
}

fn peel_string(env: &mut JNIEnv, jstring: &JString) -> String {
    env.get_string(jstring).expect("Couldn't get java string").into()
}

fn new_json_string<'a, T>(env: JNIEnv<'a>, o: &T) -> JString<'a>
    where T: ?Sized + Serialize
{
    let str = serde_json::to_string(o).expect("Couldn't serialize response");
    env.new_string(str).expect("Couldn't create java string").into()
}

fn new_jstring<'a>(env: JNIEnv<'a>, s: &str) -> JString<'a> {
    env.new_string(s).expect("Couldn't create java string").into()
}
