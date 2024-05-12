package io.cjlee.gitnote

import com.intellij.AppTopics
import com.intellij.openapi.editor.event.EditorFactoryEvent
import com.intellij.openapi.editor.event.EditorFactoryListener
import com.intellij.openapi.fileEditor.FileDocumentManager
import com.intellij.openapi.roots.ProjectRootManager
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.ProcessCoreConnector

class NoteEditorFactoryListener : EditorFactoryListener {
    private val registered: MutableSet<NoteDocumentListener> = hashSetOf()

    override fun editorCreated(event: EditorFactoryEvent) {
        // TODO : the basePath cannot be matched if project has nested(git submodule)
        //   => Discovering repository should depends on file path
        val project = event.editor.project ?: return
        val basePath = project.basePath ?: return
        val file = FileDocumentManager.getInstance().getFile(event.editor.document) ?: return
        if (!file.isValid || !ProjectRootManager.getInstance(project).fileIndex.isInContent(file)) return

        val editor = event.editor
        val handler = CoreHandler(ProcessCoreConnector(basePath))
        val documentListener = NoteDocumentListener(editor, handler, file)
        registered.add(documentListener)

        editor.document.addDocumentListener(documentListener.also { registered.add(it) })
    }

    override fun editorReleased(event: EditorFactoryEvent) {
        val file = FileDocumentManager.getInstance().getFile(event.editor.document) ?: return
        val documentListener = registered.find { it.file.path == file.path } ?: return
        event.editor.document.removeDocumentListener(documentListener)
    }
}
