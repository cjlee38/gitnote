package io.cjlee.gitnote

import com.intellij.openapi.Disposable
import com.intellij.openapi.components.Service
import com.intellij.openapi.editor.EditorFactory

@Service
class GitNoteService : Disposable {
    init {
        val gitNoteEditorFactoryListener = GitNoteEditorFactoryListener()
        EditorFactory.getInstance().addEditorFactoryListener(gitNoteEditorFactoryListener, this)
    }

    override fun dispose() {
        /*
        Do Nothing
         */
    }
}
