package io.cjlee.gitnote

import com.intellij.openapi.editor.Editor
import com.intellij.openapi.editor.event.BulkAwareDocumentListener
import com.intellij.openapi.editor.event.DocumentEvent
import com.intellij.openapi.vfs.VirtualFile
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Note

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

    // TODO : how to invoke in bulk mode ?
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

        note.messages.groupBy { it.line }
            .forEach { (line, messages) ->
                markupModel.addLineHighlighter(null, line - 1, 0).apply {
                    gutterIconRenderer = NoteIconGutterIconRenderer(file.path, messages, handler)
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
