package io.cjlee.gitnote

import com.intellij.openapi.Disposable
import com.intellij.openapi.components.Service
import com.intellij.openapi.components.service
import com.intellij.openapi.editor.EditorFactory

@Service
class CustomLineMarkerService : Disposable {
    init {
        println("======CustomLineMarkerService initialized")
        val noteEditorFactoryListener = NoteEditorFactoryListener()
        EditorFactory.getInstance().addEditorFactoryListener(noteEditorFactoryListener, this)
    }

    override fun dispose() {
        println("======CustomLineMarkerService disposed")
        // No need to manually unregister the editorCustomizer, it will be disposed automatically
    }
}
