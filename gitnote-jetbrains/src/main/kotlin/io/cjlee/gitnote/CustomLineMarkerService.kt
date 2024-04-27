package io.cjlee.gitnote

import com.intellij.openapi.Disposable
import com.intellij.openapi.components.Service
import com.intellij.openapi.editor.EditorFactory

@Service
class CustomLineMarkerService : Disposable {
    init {
        val noteEditorFactoryListener = NoteEditorFactoryListener()
        EditorFactory.getInstance().addEditorFactoryListener(noteEditorFactoryListener, this)
    }

    override fun dispose() {
        /*
        Do Nothing
         */
    }
}
