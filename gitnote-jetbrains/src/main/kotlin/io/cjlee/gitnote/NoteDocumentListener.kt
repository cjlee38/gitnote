package io.cjlee.gitnote

import com.intellij.openapi.editor.Editor
import com.intellij.openapi.editor.event.BulkAwareDocumentListener
import com.intellij.openapi.editor.event.DocumentEvent
import com.intellij.openapi.vfs.VirtualFile
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Message
import io.cjlee.gitnote.core.Note
import java.time.LocalDateTime

class NoteDocumentListener(
    private val editor: Editor,
    private val handler: CoreHandler,
    val file: VirtualFile
) : BulkAwareDocumentListener {
    private lateinit var note: Note

    init {
        val note = handler.read(file.path)
        if (note != null) {
            this.note = note
            refreshGutter(editor)
        }
    }

    override fun documentChanged(event: DocumentEvent) {
        println("======documentChanged")

        if (!event.document.isInBulkUpdate) {
            return
//            return documentChangedNonBulk(event)
        }

        refreshGutter(editor)
    }

    override fun documentChangedNonBulk(event: DocumentEvent) {
        println("======documentChangedNonBulk")

        refreshGutter(editor)
    }

    fun refreshGutter(editor: Editor) {
        println("======refreshGutter")

        val markupModel = editor.markupModel
        // Clear existing gutter icons and highlights
        editor.markupModel.removeAllHighlighters()

        if (!::note.isInitialized) {
            // for test
            val startLine = 3
            val endLine = 5
            markupModel.addLineHighlighter(null, startLine, 0).apply {
                val mockMessage = Message("id", 3, 5, "mock message", listOf(), LocalDateTime.now())
                gutterIconRenderer = NoteIconGutterIconRenderer(file.path, mockMessage, handler)
//                lineMarkerRenderer = NoteStrokeLineMarkerRenderer(startLine, endLine)
            }
        } else {
            note.messages.forEach {
                val startLine = it.startLine
                val endLine = it.endLine
                markupModel.addLineHighlighter(null, startLine - 1, 0).apply {
                    gutterIconRenderer = NoteIconGutterIconRenderer(file.path, it, handler)
//                    lineMarkerRenderer = NoteStrokeLineMarkerRenderer(startLine, endLine)
                }
            }
        }
    }

    override fun equals(other: Any?): Boolean {
        return this.file.path == (other as? NoteDocumentListener)?.file?.path
    }

    override fun hashCode(): Int {
        return 31 * file.path.hashCode()
    }
}
