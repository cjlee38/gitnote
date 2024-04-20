package io.cjlee.gitnote

import com.intellij.openapi.components.Service
import com.intellij.openapi.components.service
import com.intellij.openapi.Disposable
import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.editor.EditorFactory

@Service
class CustomLineMarkerService : Disposable {
    private val LOG = Logger.getInstance(this.javaClass)
    init {
        println("======CustomLineMarkerService initialized")

        // Register the editorCustomizer and pass `this` as the Disposable
        val editorCustomizer = EditorCustomizer()
        EditorFactory.getInstance().addEditorFactoryListener(editorCustomizer, this)
    }

    override fun dispose() {
        println("======CustomLineMarkerService disposed")
        // No need to manually unregister the editorCustomizer, it will be disposed automatically
    }

    companion object {
        // Provides a static method to get instance of the service
        val instance: CustomLineMarkerService
            get() = service<CustomLineMarkerService>().also {
                println("=================instance get!!!===================")
            }
    }
}
