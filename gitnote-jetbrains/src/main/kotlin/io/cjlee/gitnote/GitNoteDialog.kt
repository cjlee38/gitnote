package io.cjlee.gitnote

import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.convertValue
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.intellij.openapi.application.invokeLater
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogWrapper
import com.intellij.openapi.util.Disposer
import com.intellij.ui.components.JBScrollPane
import com.intellij.util.ui.JBUI
import com.jetbrains.rd.swing.sizeProperty
import io.cjlee.gitnote.jcef.GitNoteViewerWindow
import io.cjlee.gitnote.jcef.JcefViewerWindowService
import io.cjlee.gitnote.jcef.protocol.ProtocolHandler
import java.awt.event.KeyEvent
import javax.swing.Action
import javax.swing.JComponent
import javax.swing.JPanel
import javax.swing.JViewport
import javax.swing.KeyStroke
import javax.swing.ScrollPaneConstants
import javax.swing.SwingUtilities

class GitNoteDialog(
    private val project: Project?,
    private val protocolHandlers: Map<String, ProtocolHandler>
) : DialogWrapper(true) {
    companion object {
        private const val WIDTH = 430
        private const val HEIGHT = 110
        private const val MARGIN_WIDTH_FOR_CONTENT = 20
        private const val MARGIN_HEIGHT_FOR_CONTENT = 20
        private const val MARGIN_WIDTH_FOR_DIALOG = 50
        private const val MARGIN_HEIGHT_FOR_DIALOG = 70
    }

    private val mapper = jacksonObjectMapper().registerModule(JavaTimeModule())
    private lateinit var window: GitNoteViewerWindow

    init {
        title = "Gitnote"
        setSize(WIDTH, HEIGHT) // initial size before rendered

        init()
    }

    override fun createCenterPanel(): JComponent {
        requireNotNull(project)

        val service = project.getService(JcefViewerWindowService::class.java)
        val windowDisposalProtocolHandler = object : ProtocolHandler {
            override fun handle(data: Any?): ProtocolHandler.Response {
                SwingUtilities.invokeLater { close(0) }
                return ProtocolHandler.Response()
            }
        }

        data class ResizeDimension(
            val width: Int,
            val height: Int,
        )

        val windowResizeProtocolHandler = object : ProtocolHandler {
            override fun handle(data: Any?): ProtocolHandler.Response {
                val dimension = mapper.convertValue<ResizeDimension>(data!!)
                val width = dimension.width.coerceAtLeast(WIDTH)
                val height = dimension.height.coerceAtLeast(HEIGHT)

                window.content.preferredSize = JBUI.size(width + MARGIN_WIDTH_FOR_CONTENT, height + MARGIN_HEIGHT_FOR_CONTENT)

                val dialogWindow = SwingUtilities.getWindowAncestor(window.content)
                dialogWindow.size = JBUI.size(width + MARGIN_WIDTH_FOR_DIALOG, height + MARGIN_HEIGHT_FOR_DIALOG)
                return ProtocolHandler.Response()
            }
        }
        val handlers =
            protocolHandlers + ("window/close" to windowDisposalProtocolHandler) + ("window/resize" to windowResizeProtocolHandler)

        this.window = service.newWindow(handlers)
        Disposer.register(this.disposable, this.window.webView)

        return window.content.apply {
            registerKeyboardAction(
                { SwingUtilities.invokeLater { close(0) } },
                KeyStroke.getKeyStroke(KeyEvent.VK_ESCAPE, 0),
                JComponent.WHEN_ANCESTOR_OF_FOCUSED_COMPONENT
            )
        }
    }

    override fun createActions(): Array<Action> {
        // hide OK/CANCEL button
        return emptyArray()
    }

    // for dialog persistence (particularly for resizing)
    override fun getDimensionServiceKey(): String {
        return "NoteDialogServiceKey"
    }
}
