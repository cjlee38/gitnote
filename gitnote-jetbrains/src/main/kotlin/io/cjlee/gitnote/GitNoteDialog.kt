package io.cjlee.gitnote

import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.convertValue
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogWrapper
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.jcef.GitNoteViewerWindow
import io.cjlee.gitnote.jcef.JcefViewerWindowService
import io.cjlee.gitnote.jcef.protocol.ProtocolMessaage
import io.cjlee.gitnote.jcef.protocol.ProtocolHandler
import java.awt.BorderLayout
import javax.swing.Action
import javax.swing.JComponent
import javax.swing.JPanel


class GitNoteDialog(
    private val project: Project?,
    private val filePath: String,
    private val handler: CoreHandler,
    private val line: Int,
    private val onDispose: () -> Unit
) : DialogWrapper(true) {

    private val mapper = jacksonObjectMapper().registerModule(JavaTimeModule())
    private lateinit var window: GitNoteViewerWindow

    init {
        title = "Gitnote"
        setSize(500, 200)
        init()
        pack()
    }

    override fun createCenterPanel(): JComponent {
        if (project == null) {
            throw IllegalStateException("project null")
        }
        val service = project.getService(JcefViewerWindowService::class.java)

        val protocolHandlers = mapOf(
            "messages/read" to object: ProtocolHandler {
                override fun handle(data: Any?): ProtocolHandler.Response {
                    val messages = handler.readMessages(filePath, line)
                        .map { ProtocolMessaage(it.line, it.message) }
                        .ifEmpty { listOf(ProtocolMessaage(line, "")) }
                    return ProtocolHandler.Response(messages)
                }
            },
            "messages/upsert" to object: ProtocolHandler {
                override fun handle(data: Any?): ProtocolHandler.Response {
                    val message =  mapper.convertValue<ProtocolMessaage>(data!!)
                    if (message.message.isEmpty()) {
                        handler.delete(filePath, message.line)
                    }
                    val addResponse = handler.add(filePath, message.line, message.message)
                    if (addResponse.isSuccess) {
                        return ProtocolHandler.Response()
                    }
                    val updateResponse = handler.update(filePath, message.line, message.message)
                    if (updateResponse.isSuccess) {
                        return ProtocolHandler.Response()
                    }
                    return ProtocolHandler.Response(error = "Failed to update message : ${updateResponse.text}")
                }
            },
            "messages/delete" to object: ProtocolHandler {
                override fun handle(data: Any?): ProtocolHandler.Response {
                    val message =  mapper.convertValue<ProtocolMessaage>(data!!)
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
            pack()
            add(window.content, BorderLayout.CENTER)
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
