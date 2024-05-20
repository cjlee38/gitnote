package io.cjlee.gitnote

import com.intellij.openapi.editor.event.EditorFactoryEvent
import com.intellij.openapi.editor.event.EditorFactoryListener
import com.intellij.openapi.fileEditor.FileDocumentManager
import com.intellij.openapi.roots.ProjectRootManager
import com.intellij.openapi.vfs.VirtualFile
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.ProcessCoreConnector

class GitNoteEditorFactoryListener : EditorFactoryListener {
    private val registered: MutableSet<GitNoteDocumentListener> = hashSetOf()

    override fun editorCreated(event: EditorFactoryEvent) {
        val project = event.editor.project ?: return

        val file = FileDocumentManager.getInstance().getFile(event.editor.document) ?: return
        if (!file.isValid || !ProjectRootManager.getInstance(project).fileIndex.isInContent(file)) return

        val editor = event.editor
        val projectPath = findBasePath(file) ?: return
        val handler = CoreHandler(ProcessCoreConnector(projectPath))
        val documentListener = GitNoteDocumentListener(editor, handler, file)
        registered.add(documentListener)

        editor.document.addDocumentListener(documentListener.also { registered.add(it) })
    }

    private fun findBasePath(file: VirtualFile): String? {
        var f = file.parent
        while (f != null) {
            val found = f.findChild(".git")
            if (found != null) {
                return found.canonicalPath
            }
            f = f.parent
        }
        return null
    }

    override fun editorReleased(event: EditorFactoryEvent) {
        val file = FileDocumentManager.getInstance().getFile(event.editor.document) ?: return
        val documentListener = registered.find { it.file.path == file.path } ?: return
        documentListener.dispose()
        event.editor.document.removeDocumentListener(documentListener)
    }
}
