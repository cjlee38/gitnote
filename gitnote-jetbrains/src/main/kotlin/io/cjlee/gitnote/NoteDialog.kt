package io.cjlee.gitnote

import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogWrapper
import com.intellij.ui.jcef.JBCefJSQuery
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Message
import io.cjlee.gitnote.jcef.JcefViewerWindowService
import io.cjlee.gitnote.jcef.protocol.MessageProtocolHandler
import javax.swing.Action
import javax.swing.JComponent
import javax.swing.JPanel

class NoteDialog(
    private val project: Project?,
    private val handler: CoreHandler,
    private val messages: List<Message>
) : DialogWrapper(true) {

    init {
        title = "Gitnote"
//        setSize(800, 600)
        init()
    }

    override fun createCenterPanel(): JComponent {
        if (project == null) {
            throw IllegalStateException("project null")
        }
        // TODO : how to pass the messages to window to shows up?
        val messages = listOf("mock message 1", "mock message 2", "mock message3")
        val service = project.getService(JcefViewerWindowService::class.java)

        val protocolHandlers = mapOf(
            "initialMessages" to object: MessageProtocolHandler {
                override fun handle(data: Any?): JBCefJSQuery.Response {
                    val resp = jacksonObjectMapper().writeValueAsString(messages)
                    return JBCefJSQuery.Response(resp)
                }
            }
        )
        val window = service.newWindow(protocolHandlers)

        return JPanel().apply {
            add(window.content)
        }
    }

    override fun createActions(): Array<Action> {
        return emptyArray()
    }
}
