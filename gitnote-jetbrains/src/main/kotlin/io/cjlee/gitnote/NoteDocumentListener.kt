package io.cjlee.gitnote

import com.intellij.openapi.editor.Document
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.editor.event.BulkAwareDocumentListener
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
) : BulkAwareDocumentListener.Simple {
    private var note: Note? = null
    private val markupModelCache = MarkupModelCache(editor.markupModel)
    private val onDispose = { this.refreshGutter(force = true) }
    private val debouncer = Debouncer()

    init {
        refreshGutter(force = true)
        setupHoverIcon()
    }

    // TODO : bulk aware doesn't work as expected now, so here I implmeneted a very simple debouncer.
    //   If bulk aware works, I can remove this debouncer, or even though coroutine might helps to handle this.
    class Debouncer {
        private var lastRun = 0L
        private val delay = 1000L

        fun passed(): Boolean {
            val now = System.currentTimeMillis()
            return (now - lastRun > delay).also { if (it) lastRun = now }
        }
    }

    override fun afterDocumentChange(document: Document) {
        val force = debouncer.passed().also { println("debouncer said $it") }
        refreshGutter(force)
    }

    private fun refreshGutter(force: Boolean) {
        handler.read(file.path)?.let {
            this.note = handler.read(file.path, force)
            markupModelCache.removeAllIcons()
            addMessageIcons(onDispose)
        }
    }

    private fun addMessageIcons(onDispose: () -> Unit) {
        note?.let { note ->
            note.messages
                .groupBy { it.line }
                .forEach { (line, messages) ->
                    try {
                        markupModelCache.addIcon(line - 1, NoteGutterIconRenderer(file.path, handler, messages, onDispose))
                    } catch (ignore: Exception) {
                    }
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
                    markupModelCache.removeIcon(prevLine, currentHighlighter)
                    currentHighlighter = null
                }

                // Check if mouse is over the gutter area
                if (mouseEvent.x > gutterBounds.width) {
                    return
                }

                val line = editor.xyToLogicalPosition(mouseEvent.point).line + 1
                if (markupModelCache.contains(line)) {
                    return
                }

                try {
                    prevLine = line - 1
                    currentHighlighter =
                        markupModelCache.addIcon(line - 1, AddNoteGutterIconRenderer(file.path, handler, line, onDispose))
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
            val highlighter = markupModel.addLineHighlighter(null, line, 0)
            highlighter.gutterIconRenderer = gutterIconRenderer
            highlighters[line] = highlighter
            return highlighter
        }

        fun removeAllIcons() {
            markupModel.removeAllHighlighters()
            highlighters.clear()
        }

        fun removeIcon(line: Int, prev: RangeHighlighter? = null) {
            val highlighter = highlighters[line] ?: return
            if (prev != null && prev == highlighter) {
                markupModel.removeHighlighter(highlighter)
                highlighters.remove(line)
            }
        }

        fun contains(line: Int): Boolean {
            return highlighters.containsKey(line)
        }
    }
}
