package io.cjlee.gitnote

import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.convertValue
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.intellij.openapi.editor.Document
import com.intellij.openapi.editor.event.BulkAwareDocumentListener
import com.intellij.openapi.editor.event.DocumentEvent
import com.intellij.openapi.editor.ex.EditorEx
import com.intellij.openapi.editor.ex.RangeHighlighterEx
import com.intellij.openapi.editor.markup.HighlighterLayer
import com.intellij.openapi.editor.markup.HighlighterTargetArea
import com.intellij.openapi.fileEditor.FileDocumentManager
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.util.ui.update.MergingUpdateQueue
import com.intellij.util.ui.update.Update
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Note
import io.cjlee.gitnote.jcef.protocol.ProtocolHandler
import io.cjlee.gitnote.jcef.protocol.ProtocolMessaage
import javax.swing.SwingUtilities


class GitNoteDocumentListener(
    private val editor: EditorEx,
    private val handler: CoreHandler,
    private val file: VirtualFile
) : BulkAwareDocumentListener {
    private lateinit var note: Note
    private val mapper = jacksonObjectMapper().registerModule(JavaTimeModule())
    private val reloadOnEventThread = { SwingUtilities.invokeLater { this.reload() } }
    private val queue = MergingUpdateQueue("GitNoteQueue", 100, true, null)

    private val lineHighlighters = mutableSetOf<RangeHighlighterEx>()

    init {
        reloadOnEventThread()
        SwingUtilities.invokeLater {
            val iconVisibility = IconVisibility(lineHighlighters)
            editor.addEditorMouseListener(iconVisibility)
            editor.addEditorMouseMotionListener(iconVisibility)
        }
    }

    override fun documentChanged(event: DocumentEvent) {
        queue.queue(object : Update("identity") {
            override fun run() {
                FileDocumentManager.getInstance().saveDocument(event.document)
            }
        })
        addEmptyMessageIcons(event.document)
    }

    private fun reload() {
        this.note = handler.read(file.path, force = true) ?: throw IllegalStateException("no note")
        addNoteMessageIcons(editor.document)
        addEmptyMessageIcons(editor.document)
    }

    private fun addNoteMessageIcons(document: Document) {
        lineHighlighters.clear()
        editor.markupModel.removeAllHighlighters()

        val noteLineMessages = note.messages.groupBy { it.line }
        (0 until document.lineCount).forEach { line ->
            val start = editor.document.getLineStartOffset(line)
            val end = editor.document.getLineEndOffset(line)

            val lineMessages = noteLineMessages[line]
            val protocolHandlers = createProtocolHandlers(file.path, line)
            editor.markupModel.addRangeHighlighterAndChangeAttributes(
                null,
                start,
                end,
                HighlighterLayer.LAST,
                HighlighterTargetArea.EXACT_RANGE,
                false
            ) { highlighter ->
                val gitNoteGutterIconRenderer = GitNoteGutterIconRenderer(
                    lineMessages = lineMessages ?: emptyList(),
                    protocolHandlers = protocolHandlers,
                    visible = lineMessages != null,
                    highlighter = highlighter,
                    document = document
                )
                highlighter.gutterIconRenderer = gitNoteGutterIconRenderer
            }.also { highlighter -> lineHighlighters.add(highlighter) }
        }
    }

    private fun addEmptyMessageIcons(document: Document) {
        val renderersByHasMessage = lineHighlighters.map { it.gutterIconRenderer as GitNoteGutterIconRenderer }
            .filter { !it.hasMessage }
        renderersByHasMessage.forEach {
            lineHighlighters.remove(it.highlighter)
            editor.markupModel.removeHighlighter(it.highlighter)
        }

        (0 until document.lineCount).forEach { line ->
            val start = editor.document.getLineStartOffset(line)
            val end = editor.document.getLineEndOffset(line)

            if (lineHighlighters.any { (it.gutterIconRenderer as GitNoteGutterIconRenderer).line == line }) {
                return@forEach
            }

            val protocolHandlers = createProtocolHandlers(file.path, line)
            editor.markupModel.addRangeHighlighterAndChangeAttributes(
                null,
                start,
                end,
                HighlighterLayer.LAST,
                HighlighterTargetArea.EXACT_RANGE,
                false
            ) { highlighter ->
                highlighter.gutterIconRenderer = GitNoteGutterIconRenderer(
                    lineMessages = emptyList(),
                    protocolHandlers = protocolHandlers,
                    visible = false,
                    highlighter = highlighter,
                    document = document
                )
            }.also { highlighter -> lineHighlighters.add(highlighter) }
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
            "messages/insert" to object : ProtocolHandler {
                override fun handle(data: Any?): ProtocolHandler.Response {
                    val protocolMessage = mapper.convertValue<ProtocolMessaage>(data!!)
                    if (protocolMessage.message.isEmpty()) {
                        handler.delete(filePath, protocolMessage.line)
                        return ProtocolHandler.Response()
                    }
                    val response = handler.add(filePath, protocolMessage.line, protocolMessage.message)
                    if (response.isSuccess) {
                        reloadOnEventThread()
                        return ProtocolHandler.Response()
                    }
                    return ProtocolHandler.Response(error = "Failed to add message : ${response.text}")
                }
            },
            "messages/update" to object : ProtocolHandler {
                override fun handle(data: Any?): ProtocolHandler.Response {
                    val protocolMessage = mapper.convertValue<ProtocolMessaage>(data!!)
                    if (protocolMessage.message.isEmpty()) {
                        handler.delete(filePath, protocolMessage.line)
                        return ProtocolHandler.Response()
                    }

                    val response = handler.update(filePath, protocolMessage.line, protocolMessage.message)
                    if (response.isSuccess) {
                        reloadOnEventThread()
                        return ProtocolHandler.Response()
                    }
                    return ProtocolHandler.Response(error = "Failed to update message : ${response.text}")
                }
            },
            "messages/delete" to object : ProtocolHandler {
                override fun handle(data: Any?): ProtocolHandler.Response {
                    val message = mapper.convertValue<ProtocolMessaage>(data!!)
                    val deleteResponse = handler.delete(filePath, message.line)
                    if (!deleteResponse.isSuccess) {
                        return ProtocolHandler.Response(error = "Failed to delete message : ${deleteResponse.text}")
                    }
                    reloadOnEventThread()
                    return ProtocolHandler.Response()
                }
            },
        )
    }

    override fun equals(other: Any?): Boolean {
        return this.file.path == (other as? GitNoteDocumentListener)?.file?.path
    }

    override fun hashCode(): Int {
        return 31 * file.path.hashCode()
    }
}
