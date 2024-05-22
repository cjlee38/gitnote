package io.cjlee.gitnote

import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.convertValue
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.editor.Document
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.editor.event.BulkAwareDocumentListener
import com.intellij.openapi.editor.event.EditorMouseEvent
import com.intellij.openapi.editor.event.EditorMouseMotionListener
import com.intellij.openapi.editor.ex.EditorGutterComponentEx
import com.intellij.openapi.editor.markup.GutterIconRenderer
import com.intellij.openapi.editor.markup.MarkupModel
import com.intellij.openapi.editor.markup.RangeHighlighter
import com.intellij.openapi.fileEditor.FileDocumentManager
import com.intellij.openapi.vfs.VirtualFile
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Note
import io.cjlee.gitnote.jcef.protocol.ProtocolHandler
import io.cjlee.gitnote.jcef.protocol.ProtocolMessaage
import java.util.concurrent.Executors


class GitNoteDocumentListener(
    private val editor: Editor,
    private val handler: CoreHandler,
    val file: VirtualFile
) : BulkAwareDocumentListener.Simple {
    private var note: Note? = null
    private val markupModelCache = MarkupModelCache(editor.markupModel)
    private val mapper = jacksonObjectMapper().registerModule(JavaTimeModule())
    private lateinit var document: Document
    private val onDispose = {
        ApplicationManager.getApplication().invokeLater { this.refreshGutter(force = true) }
    }
    private val debouncer = Debouncer()
    private val executor = Executors.newSingleThreadScheduledExecutor()

    init {
        refreshGutter(force = true)
        setupHoverIcon()
        executor.scheduleWithFixedDelay({ refreshGutter(force = true) }, 1, 1, java.util.concurrent.TimeUnit.SECONDS)
    }

    // TODO : bulk aware doesn't work as expected now, so here I implmeneted a very simple debouncer.
    //   If bulk aware works somehow, I can remove this debouncer, or even though coroutine might helps to handle this.
    class Debouncer {
        private var lastRun = 0L
        private val delay = 1000L

        fun passed(): Boolean {
            val now = System.currentTimeMillis()
            return (now - lastRun > delay).also { if (it) lastRun = now }
        }
    }

    fun dispose() {
        executor.shutdown()
        markupModelCache.removeAllIcons()
    }

    override fun afterDocumentChange(document: Document) {
        val manager = FileDocumentManager.getInstance()
        if (manager.isDocumentUnsaved(document)) {
            manager.saveDocument(document)
        }
        val force = debouncer.passed()
        refreshGutter(force)
    }

    private fun refreshGutter(force: Boolean) {
        handler.read(file.path)?.let {
            this.note = handler.read(file.path, force)
            markupModelCache.removeAllIcons()
            addMessageIcons()
        }
    }

    private fun addMessageIcons() {
        note?.let { note ->
            note.messages
                .groupBy { it.line }
                .forEach { (line, messages) ->
                    try {
                        println("addMessageIcons : $line")
                        val protocolHandlers = createProtocolHandlers(file.path, line)
                        markupModelCache.addIcon(line - 1, GitNoteGutterIconRenderer(messages, protocolHandlers))
                    } catch (ignore: Exception) {
                    }
                }
        }
    }

    private fun createProtocolHandlers(filePath: String, line: Int): Map<String, ProtocolHandler> {
        return mapOf(
            "messages/read" to object : ProtocolHandler {
                override fun handle(data: Any?): ProtocolHandler.Response {
                    val messages = handler.readMessages(filePath, line)
                        .map { ProtocolMessaage(it.line, it.message) }
                        .ifEmpty { listOf(ProtocolMessaage(line, "")) }
                    return ProtocolHandler.Response(messages)
                }
            },
            "messages/upsert" to object : ProtocolHandler {
                override fun handle(data: Any?): ProtocolHandler.Response {
                    val message = mapper.convertValue<ProtocolMessaage>(data!!)
                    if (message.message.isEmpty()) {
                        handler.delete(filePath, message.line)
                    }
                    val addResponse = handler.add(filePath, message.line, message.message)
                    if (addResponse.isSuccess) {
                        onDispose()
                        return ProtocolHandler.Response()
                    }
                    val updateResponse = handler.update(filePath, message.line, message.message)
                    if (updateResponse.isSuccess) {
                        onDispose()
                        return ProtocolHandler.Response()
                    }
                    return ProtocolHandler.Response(error = "Failed to add or update message : ${updateResponse.text}")
                }
            },
            "messages/delete" to object : ProtocolHandler {
                override fun handle(data: Any?): ProtocolHandler.Response {
                    val message = mapper.convertValue<ProtocolMessaage>(data!!)
                    val deleteResponse = handler.delete(filePath, message.line)
                    if (!deleteResponse.isSuccess) {
                        return ProtocolHandler.Response(error = "Failed to delete message : ${deleteResponse.text}")
                    }
                    onDispose()
                    return ProtocolHandler.Response()
                }
            },
        )
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
                    currentHighlighter = markupModelCache.addIcon(
                        line - 1,
                        AddGitNoteGutterIconRenderer(line, createProtocolHandlers(file.path, line))
                    )
                } catch (ignore: Exception) {
                }
            }

        })
    }

    override fun equals(other: Any?): Boolean {
        return this.file.path == (other as? GitNoteDocumentListener)?.file?.path
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
