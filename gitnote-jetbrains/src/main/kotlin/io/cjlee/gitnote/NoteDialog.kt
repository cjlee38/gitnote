package io.cjlee.gitnote

import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogWrapper
import com.intellij.ui.jcef.JBCefJSQuery
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Message
import io.cjlee.gitnote.jcef.JcefViewerWindowService
import io.cjlee.gitnote.jcef.protocol.MessageProtocolHandler
import java.awt.GridLayout
import javax.swing.Action
import javax.swing.JComponent
import javax.swing.JPanel

class NoteDialog(
    private val project: Project?,
    private val filePath: String,
    private val handler: CoreHandler,
    private val messages: List<Message>,
    private val line: Int,
) : DialogWrapper(true) {

    private val mapper = jacksonObjectMapper().registerModule(JavaTimeModule())

    init {
        title = "Gitnote"
        setSize(800, 600)
        init()
    }

    override fun createCenterPanel(): JComponent {
        if (project == null) {
            throw IllegalStateException("project null")
        }
        val service = project.getService(JcefViewerWindowService::class.java)

        // TODO : refresh after update or delete or add
        val protocolHandlers = mapOf(
            "messages/read" to object: MessageProtocolHandler {
                override fun handle(data: Any?): JBCefJSQuery.Response {
                    val messages = handler.read(filePath)?.let { it.messages.filter { it.line == line } } ?: emptyList()
                    val resp = mapper.writeValueAsString(messages)
                    return JBCefJSQuery.Response(resp)
                }
            },
            "messages/update" to object: MessageProtocolHandler {
                override fun handle(data: Any?): JBCefJSQuery.Response {
                    val message = mapper.readValue<Message>(mapper.writeValueAsString(data)) // temporary
                    handler.update(filePath, message.line, message.message)
                    return JBCefJSQuery.Response("OK")
                }
            },
            "messages/delete" to object: MessageProtocolHandler {
                override fun handle(data: Any?): JBCefJSQuery.Response {
                    val message = mapper.readValue<Message>(mapper.writeValueAsString(data))
                    handler.delete(filePath, message.line)
                    dispose()
                    return JBCefJSQuery.Response("OK")
                }
            },
        )
        val window = service.newWindow(protocolHandlers)

        return JPanel().apply {
            layout = GridLayout(0, 1)
            add(window.content)
        }
    }

    override fun createActions(): Array<Action> {
        return emptyArray()
    }
}
