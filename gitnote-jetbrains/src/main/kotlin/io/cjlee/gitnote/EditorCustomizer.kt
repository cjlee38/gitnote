package io.cjlee.gitnote

import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.editor.event.EditorFactoryEvent
import com.intellij.openapi.editor.event.EditorFactoryListener

class EditorCustomizer : EditorFactoryListener {
    private val LOG = Logger.getInstance(this.javaClass)

    override fun editorCreated(event: EditorFactoryEvent) {
        println("======editorCreated")

        val editor = event.editor
        initializeGutter(editor)
        editor.document.addDocumentListener(DebouncedDocumentListener(editor))
    }

    override fun editorReleased(event: EditorFactoryEvent) {
        println("======editorReleased")

        val editor = event.editor
        editor.document.removeDocumentListener(DebouncedDocumentListener(editor))
    }

    private fun initializeGutter(editor: Editor) {
        println("======initializeGutter")

        // Initially setup or refresh the gutter icon and background color when an editor is created
        // Call refreshGutter with example line numbers
        DebouncedDocumentListener(editor).refreshGutter(editor, 2, 4) // Example line numbers, adjust as needed
    }
}
