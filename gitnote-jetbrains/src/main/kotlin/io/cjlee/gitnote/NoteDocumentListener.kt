package io.cjlee.gitnote

import com.intellij.openapi.editor.Editor
import com.intellij.openapi.editor.event.BulkAwareDocumentListener
import com.intellij.openapi.editor.event.DocumentEvent
import com.intellij.openapi.editor.event.EditorMouseEvent
import com.intellij.openapi.editor.event.EditorMouseMotionListener
import com.intellij.openapi.editor.ex.EditorGutterComponentEx
import com.intellij.openapi.editor.markup.GutterIconRenderer
import com.intellij.openapi.editor.markup.MarkupModel
import com.intellij.openapi.editor.markup.RangeHighlighter
import com.intellij.openapi.vfs.VirtualFile
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Note


class NoteDocumentListener(
    private val editor: Editor,
    private val handler: CoreHandler,
    val file: VirtualFile
) : BulkAwareDocumentListener {
    private var note: Note? = null
    private val markupModelCache = MarkupModelCache(editor.markupModel)
    private val onDispose = { this.refreshGutter() }

    init {
        val note = handler.read(file.path)
        if (note != null) {
            this.note = note
            refreshGutter()
        }
        setupHoverIcon()
    }

    // TODO : how to invoke in bulk mode ?
    override fun documentChanged(event: DocumentEvent) {
        println("======documentChanged")

        if (!event.document.isInBulkUpdate) {
            return
//            return documentChangedNonBulk(event)
        }

        refreshGutter()
    }

    override fun documentChangedNonBulk(event: DocumentEvent) {
        println("======documentChangedNonBulk")

        refreshGutter()
    }

    private fun refreshGutter() {
        println("======refreshGutter")
        markupModelCache.removeAllIcons()
        addMessageIcons(onDispose)
    }

    private fun addMessageIcons(onDispose: () -> Unit) {
        note?.let { note ->
            note.messages
                .groupBy { it.line }
                .forEach { (line, _) ->
                    markupModelCache.addIcon(line - 1, NoteGutterIconRenderer(file.path, handler, line, onDispose))
                }
        }
    }

    private fun setupHoverIcon() {
        editor.addEditorMouseMotionListener(object : EditorMouseMotionListener {
            var prevLine = -1
            var currentHighlighter: RangeHighlighter? = null

            override fun mouseMoved(e: EditorMouseEvent) {
                val gutterComponent = editor.gutter as EditorGutterComponentEx
                val gutterBounds = gutterComponent.bounds
                val mouseEvent = e.mouseEvent

                if (currentHighlighter != null && prevLine != -1) {
                    markupModelCache.removeIcon(prevLine)
                    currentHighlighter = null
                }

                // Check if mouse is over the gutter area
                if (mouseEvent.x > gutterBounds.width) {
                    return
                }

                val line = editor.xyToLogicalPosition(mouseEvent.point).line
                if (markupModelCache.contains(line)) {
                    return
                }

                try {
                    prevLine = line
                    currentHighlighter =
                        markupModelCache.addIcon(line, AddNoteGutterIconRenderer(file.path, handler, line, onDispose))
                } catch (ignore: Exception) {
                }
            }

        })
    }

    override fun equals(other: Any?): Boolean {
        return this.file.path == (other as? NoteDocumentListener)?.file?.path
    }

    override fun hashCode(): Int {
        return 31 * file.path.hashCode()
    }

    class MarkupModelCache(private val markupModel: MarkupModel) {
        private val highlighters = mutableMapOf<Int, RangeHighlighter>()

        fun addIcon(line: Int, gutterIconRenderer: GutterIconRenderer?): RangeHighlighter? {
            if (contains(line)) {
                return null
            }
            // TODO : check layer effects
            val highlighter = markupModel.addLineHighlighter(line, 0, null)
            highlighter.gutterIconRenderer = gutterIconRenderer
            highlighters[line] = highlighter
            return highlighter
        }

        fun removeAllIcons() {
            highlighters.values.forEach { markupModel.removeHighlighter(it) }
            highlighters.clear()
        }

        fun removeIcon(line: Int) {
            val highlighter = highlighters[line] ?: return
            markupModel.removeHighlighter(highlighter)
            highlighters.remove(line)
        }

        fun contains(line: Int): Boolean {
            return highlighters.containsKey(line)
        }
    }
}
