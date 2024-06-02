package io.cjlee.gitnote

import com.intellij.openapi.editor.EditorKind
import com.intellij.openapi.editor.event.EditorFactoryEvent
import com.intellij.openapi.editor.event.EditorFactoryListener
import com.intellij.openapi.editor.ex.EditorEx
import com.intellij.openapi.fileEditor.FileDocumentManager
import com.intellij.openapi.roots.ProjectRootManager
import com.intellij.openapi.vfs.VirtualFile
import io.cjlee.gitnote.core.CoreHandlerFactory

class GitNoteEditorFactoryListener : EditorFactoryListener {
    private val registered: MutableSet<GitNoteDocumentListener> = hashSetOf()

    override fun editorCreated(event: EditorFactoryEvent) {
        val editor = event.editor
        val project = editor.project ?: return
        if (editor.editorKind != EditorKind.MAIN_EDITOR) return

        val file = FileDocumentManager.getInstance().getFile(editor.document) ?: return
        if (!file.isValid || !ProjectRootManager.getInstance(project).fileIndex.isInContent(file)) return


        val projectPath = findBasePath(file) ?: return
        try {
            val handler = CoreHandlerFactory.get(projectPath)
            val documentListener = GitNoteDocumentListener(editor as EditorEx, handler, file)
            if (registered.add(documentListener)) {
                editor.document.addDocumentListener(documentListener.also { registered.add(it) })
            }
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }

    private fun findBasePath(file: VirtualFile): String? {
        var f = file.parent
        while (f != null) {
            val found = f.findChild(".git")
            if (found != null) {
                return f.canonicalPath
            }
            f = f.parent
        }
        return null
    }

    // TODO : when document listener is busy, error occurs if trying to remove it.
    override fun editorReleased(event: EditorFactoryEvent) {
//        val file = FileDocumentManager.getInstance().getFile(event.editor.document) ?: return
//        val documentListener = registered.find { it.file.path == file.path } ?: return
//        event.editor.isDisposed
//        if (registered.remove(documentListener)) {
//            documentListener.dispose()
//            event.editor.document.removeDocumentListener(documentListener)
//        }
    }
}
