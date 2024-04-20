package io.cjlee.gitnote

import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.editor.Document
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.editor.event.BulkAwareDocumentListener
import com.intellij.openapi.editor.event.DocumentEvent
import com.intellij.openapi.fileEditor.FileDocumentManager

class DebouncedDocumentListener(private val editor: Editor) : BulkAwareDocumentListener {
    private val LOG = Logger.getInstance(this.javaClass)

    override fun documentChanged(event: DocumentEvent) {
        println("======documentChanged")

        if (!event.document.isInBulkUpdate) {
            return documentChangedNonBulk(event)
        }

        refreshGutter(editor, 2, 4)
    }

    override fun documentChangedNonBulk(event: DocumentEvent) {
        println("======documentChangedNonBulk")

        refreshGutter(editor, 2, 4)
    }

    fun refreshGutter(editor: Editor, startLine: Int, endLine: Int) {
        println("======refreshGutter")
        if (!isDocumentValid(editor.document)) {
            return
        }

        val markupModel = editor.markupModel
        // Clear existing gutter icons and highlights
        editor.markupModel.removeAllHighlighters()

        markupModel.addLineHighlighter(null, startLine, 0).apply {
            gutterIconRenderer = NoteIconGutterIconRenderer("custom line on $startLine")
            lineMarkerRenderer = NoteStrokeLineMarkerRenderer(startLine, endLine)
        }
    }

    private fun isDocumentValid(document: Document): Boolean {
        return FileDocumentManager.getInstance().getFile(document) != null && document.lineCount > 0
    }
}
