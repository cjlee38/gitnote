// use crate::handlers::NoteHandler;
// use crate::stdio::write_out;
//
// struct Cli {
//     note_handler: NoteHandler,
// }
//
// impl Cli {
//     pub fn new(note_handler: NoteHandler) -> Self {
//         Self { note_handler }
//     }
//
//     pub fn add_note(&self, file_name: String, line: usize, message: String) -> anyhow::Result<()> {
//         self.note_handler.add_note(file_name, line - 1, message)?;
//         write_out(&format!(
//             "Successfully added comment for {:?} in range {}",
//             &file_name,
//             line + 1
//         ));
//         Ok(())
//     }
// }
