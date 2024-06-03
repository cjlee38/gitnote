package io.cjlee.gitnote

import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogWrapper
import com.intellij.util.ui.JBUI
import io.cjlee.gitnote.jcef.GitNoteViewerWindow
import io.cjlee.gitnote.jcef.JcefViewerWindowService
import io.cjlee.gitnote.jcef.protocol.ProtocolHandler
import java.awt.BorderLayout
import java.awt.event.KeyEvent
import javax.swing.Action
import javax.swing.JComponent
import javax.swing.JPanel
import javax.swing.KeyStroke
import javax.swing.SwingUtilities
import javax.swing.border.Border

class GitNoteDialog(
    private val project: Project?,
    private val protocolHandlers: Map<String, ProtocolHandler>
) : DialogWrapper(true) {
    companion object {
        const val WIDTH = 430
        const val HEIGHT = 120
    }

    private lateinit var window: GitNoteViewerWindow

    init {
        title = "Gitnote"
        setSize(WIDTH, HEIGHT)

        init()
        pack()
    }

    override fun createContentPaneBorder(): Border {
        return JBUI.Borders.empty(0, 12, 8, 12)
    }

    override fun createCenterPanel(): JComponent {
        if (project == null) {
            throw IllegalStateException("project null")
        }
        val service = project.getService(JcefViewerWindowService::class.java)
        val windowDisposalProtocolHandler = object: ProtocolHandler {
            override fun handle(data: Any?): ProtocolHandler.Response {
                dispose()
                return ProtocolHandler.Response()
            }
        }
        val handlers = protocolHandlers + ("window/close" to windowDisposalProtocolHandler)
        this.window = service.newWindow(handlers)

        return JPanel().apply {
            layout = BorderLayout()
            add(window.content, BorderLayout.CENTER)
            size = JBUI.size(WIDTH, HEIGHT)
            minimumSize = JBUI.size(WIDTH, HEIGHT)
            pack()
            registerKeyboardAction(
                { dispose() },
                KeyStroke.getKeyStroke(KeyEvent.VK_ESCAPE, 0),
                JComponent.WHEN_ANCESTOR_OF_FOCUSED_COMPONENT
            )
        }
    }

    override fun dispose() {
        this.window.dispose()
        SwingUtilities.invokeLater { super.dispose() }
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
