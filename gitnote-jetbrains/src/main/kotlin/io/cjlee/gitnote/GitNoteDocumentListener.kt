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
import com.intellij.util.ui.update.Update.HIGH_PRIORITY
import com.intellij.util.ui.update.Update.LOW_PRIORITY
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Note
import io.cjlee.gitnote.jcef.protocol.ProtocolHandler
import io.cjlee.gitnote.jcef.protocol.ProtocolMessaage
import javax.swing.SwingUtilities

/**
 * This class is a main component which is responsible to interact with the editor users use.
 *
 * It listens to the document changes and updates the gutter icons and line highlights.
 */
class GitNoteDocumentListener(
    private val editor: EditorEx,
    private val handler: CoreHandler,
    private val file: VirtualFile
) : BulkAwareDocumentListener {
    private lateinit var note: Note
    private val mapper = jacksonObjectMapper().registerModule(JavaTimeModule())
    private val reloadOnEventThread = { SwingUtilities.invokeLater { this.reload() } }
    // queue for saving document not too frequently
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
        this.queue(LOW_PRIORITY) {
            FileDocumentManager.getInstance().saveDocument(event.document)
            addEmptyMessageIcons(event.document)
        }
    }

    private fun reload() {
        this.queue(HIGH_PRIORITY) {
            note = handler.read(file.path, force = true) ?: throw IllegalStateException("no note")
            addNoteMessageIcons(editor.document)
            addEmptyMessageIcons(editor.document)
        }
    }

    private fun addNoteMessageIcons(document: Document) {
        lineHighlighters.clear()
        editor.markupModel.removeAllHighlighters()

        val noteLineMessages = note.messages.groupBy { it.line }
        (0 until document.lineCount).forEach { line ->
            val height = heightOfLine(line)

            val lineMessages = noteLineMessages[line]
            val protocolHandlers = createProtocolHandlers(line)
            editor.markupModel.addRangeHighlighterAndChangeAttributes(
                null,
                height.first,
                height.second,
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
//        TODO : if highlighters keeps growing, mutable renderer would be answer, using below
//        editor.markupModel.allHighlighters
//            .map { it.gutterIconRenderer }
//            .filterIsInstance<GitNoteGutterIconRenderer>()

        lineHighlighters.map { it.gitNoteGutterIconRenderer }
            .filter { !it.hasMessage }
            .forEach {
                lineHighlighters.remove(it.highlighter)
                editor.markupModel.removeHighlighter(it.highlighter)
            }

        (0 until document.lineCount).forEach { line ->
            val height = heightOfLine(line)

            if (lineHighlighters.any { it.gitNoteGutterIconRenderer.line == line }) {
                return@forEach
            }

            val protocolHandlers = createProtocolHandlers(line)
            editor.markupModel.addRangeHighlighterAndChangeAttributes(
                null,
                height.first,
                height.second,
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

    private fun heightOfLine(line: Int) : Pair<Int, Int> {
        return (editor.document.getLineStartOffset(line) to editor.document.getLineEndOffset(line))
    }

    private val RangeHighlighterEx.gitNoteGutterIconRenderer: GitNoteGutterIconRenderer
        get() = this.gutterIconRenderer as GitNoteGutterIconRenderer

    private fun createProtocolHandlers(line: Int): Map<String, ProtocolHandler> {
        return mapOf(
            "messages/read" to object : ProtocolHandler {
                override fun handle(data: Any?): ProtocolHandler.Response {
                    val messages = handler.readMessages(file.path, line)
                        .map { ProtocolMessaage(it.line, it.message) }
                        .ifEmpty { listOf(ProtocolMessaage(line, "")) }
                    return ProtocolHandler.Response(messages)
                }
            },
            "messages/insert" to object : ProtocolHandler {
                override fun handle(data: Any?): ProtocolHandler.Response {
                    val protocolMessage = mapper.convertValue<ProtocolMessaage>(data!!)
                    if (protocolMessage.message.isEmpty()) {
                        handler.delete(file.path, protocolMessage.line)
                        return ProtocolHandler.Response()
                    }
                    val response = handler.add(file.path, protocolMessage.line, protocolMessage.message)
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
                        handler.delete(file.path, protocolMessage.line)
                        return ProtocolHandler.Response()
                    }

                    val response = handler.update(file.path, protocolMessage.line, protocolMessage.message)
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
                    val deleteResponse = handler.delete(file.path, message.line)
                    if (!deleteResponse.isSuccess) {
                        return ProtocolHandler.Response(error = "Failed to delete message : ${deleteResponse.text}")
                    }
                    reloadOnEventThread()
                    return ProtocolHandler.Response()
                }
            },
        )
    }

    private fun queue(priority: Int, act: () -> Unit) {
        queue.queue(object : Update("identity", priority) {
            override fun run() {
                act()
            }
        })
    }

    override fun equals(other: Any?): Boolean {
        return this.file.path == (other as? GitNoteDocumentListener)?.file?.path
    }

    override fun hashCode(): Int {
        return 31 * file.path.hashCode()
    }
}
