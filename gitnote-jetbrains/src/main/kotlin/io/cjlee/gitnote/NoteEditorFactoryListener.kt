package io.cjlee.gitnote

import com.intellij.openapi.editor.event.EditorFactoryEvent
import com.intellij.openapi.editor.event.EditorFactoryListener
import com.intellij.openapi.fileEditor.FileDocumentManager
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.ProcessCoreConnector

class NoteEditorFactoryListener : EditorFactoryListener {
    private val registered: MutableSet<NoteDocumentListener> = hashSetOf()

    override fun editorCreated(event: EditorFactoryEvent) {
        // TODO : the basePath cannot be matched if project has nested(git submodule)
        //   => Discovering repository should depends on file path
        val basePath = event.editor.project?.basePath ?: return
        val file = FileDocumentManager.getInstance().getFile(event.editor.document) ?: return
        if (!file.isValid) return

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
