package io.cjlee.gitnote

import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.convertValue
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogWrapper
import com.intellij.util.ui.JBUI
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.jcef.GitNoteViewerWindow
import io.cjlee.gitnote.jcef.JcefViewerWindowService
import io.cjlee.gitnote.jcef.protocol.ProtocolHandler
import io.cjlee.gitnote.jcef.protocol.ProtocolMessaage
import java.awt.BorderLayout
import javax.swing.Action
import javax.swing.JComponent
import javax.swing.JPanel
import javax.swing.border.Border

class GitNoteDialog(
    private val project: Project?,
    private val filePath: String,
    private val handler: CoreHandler,
    private val line: Int,
    private val onDispose: () -> Unit
) : DialogWrapper(true) {
    companion object {
        const val WIDTH = 430
        const val HEIGHT = 120
    }
    private val mapper = jacksonObjectMapper().registerModule(JavaTimeModule())
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

        val protocolHandlers = mapOf(
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
                        dispose()
                        return ProtocolHandler.Response()
                    }
                    val updateResponse = handler.update(filePath, message.line, message.message)
                    if (updateResponse.isSuccess) {
                        dispose()
                        return ProtocolHandler.Response()
                    }
                    return ProtocolHandler.Response(error = "Failed to update message : ${updateResponse.text}")
                }
            },
            "messages/delete" to object : ProtocolHandler {
                override fun handle(data: Any?): ProtocolHandler.Response {
                    val message = mapper.convertValue<ProtocolMessaage>(data!!)
                    val deleteResponse = handler.delete(filePath, message.line)
                    if (!deleteResponse.isSuccess) {
                        return ProtocolHandler.Response(error = "Failed to delete message : ${deleteResponse.text}")
                    }
                    dispose()
                    return ProtocolHandler.Response()
                }
            },
        )
        this.window = service.newWindow(protocolHandlers)


        return JPanel().apply {
            layout = BorderLayout()
            add(window.content, BorderLayout.CENTER)
            size = JBUI.size(WIDTH, HEIGHT)
            minimumSize = JBUI.size(WIDTH, HEIGHT)
            pack()
        }
    }

    override fun dispose() {
        onDispose()
        this.window.dispose()
        super.dispose()
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
