package io.cjlee.gitnote

import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.convertValue
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.intellij.openapi.editor.Document
import com.intellij.openapi.editor.FoldRegion
import com.intellij.openapi.editor.event.BulkAwareDocumentListener
import com.intellij.openapi.editor.event.DocumentEvent
import com.intellij.openapi.editor.ex.EditorEx
import com.intellij.openapi.editor.ex.FoldingListener
import com.intellij.openapi.editor.ex.RangeHighlighterEx
import com.intellij.openapi.editor.ex.util.EditorUtil
import com.intellij.openapi.editor.impl.event.MarkupModelListener
import com.intellij.openapi.editor.markup.HighlighterLayer
import com.intellij.openapi.editor.markup.HighlighterTargetArea
import com.intellij.openapi.editor.markup.RangeHighlighter
import com.intellij.openapi.fileEditor.FileDocumentManager
import com.intellij.openapi.util.Disposer
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

    private val lineHighlighters = mutableSetOf<RangeHighlighter>()

    init {
        reloadOnEventThread()
        SwingUtilities.invokeLater {
            val iconVisibility = IconVisibility(lineHighlighters)
            editor.addEditorMouseListener(iconVisibility)
            editor.addEditorMouseMotionListener(iconVisibility)
        }

        val disposable = Disposer.newDisposable()
        editor.markupModel.addMarkupModelListener(disposable, object : MarkupModelListener {
            override fun beforeRemoved(highlighter: RangeHighlighterEx) {
                val iconRenderer = highlighter.gutterIconRenderer as? GitNoteGutterIconRenderer ?: return
                Disposer.dispose(iconRenderer)
                lineHighlighters.remove(highlighter)
            }
        })
        EditorUtil.disposeWithEditor(editor, disposable)

        editor.foldingModel.addListener(object : FoldingListener {
            override fun onFoldRegionStateChange(region: FoldRegion) {
                reloadOnEventThread()
            }
        }, disposable)
    }

    override fun documentChanged(event: DocumentEvent) {
        this.queue(LOW_PRIORITY) {
            FileDocumentManager.getInstance().saveDocument(event.document)
            addMessageIcons(event.document)
        }
    }

    private fun reload() {
        this.queue(HIGH_PRIORITY) {
            note = handler.read(file.path, force = true) ?: throw IllegalStateException("no note")
            addMessageIcons(editor.document)
        }
    }

    private fun addMessageIcons(document: Document) {
        editor.markupModel.removeAllHighlighters()
        val messagesByLine = note.messages.groupBy { it.line }

        (0 until document.lineCount).forEach { line ->
            val isLineVisible = editor.foldingModel.allFoldRegions.none { region ->
                region.startOffset <= document.getLineStartOffset(line) &&
                        region.endOffset > document.getLineStartOffset(line) &&
                        !region.isExpanded &&
                        document.getLineNumber(region.startOffset) != line
            }

            if (isLineVisible) {
                val protocolHandlers = createProtocolHandlers(line)
                val lineMessages = messagesByLine[line]

                val start = editor.document.getLineStartOffset(line)
                val end = editor.document.getLineEndOffset(line)
                editor.markupModel.addRangeHighlighterAndChangeAttributes(
                    null,
                    start,
                    end,
                    HighlighterLayer.LAST,
                    HighlighterTargetArea.EXACT_RANGE,
                    false
                ) { highlighter ->
                    highlighter.gutterIconRenderer = GitNoteGutterIconRenderer(
                        lineMessages = lineMessages ?: emptyList(),
                        protocolHandlers = protocolHandlers,
                        visible = lineMessages != null,
                        line = line,
                        document = document
                    )
                }.also { lineHighlighters.add(it) }
            }
        }
    }

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
