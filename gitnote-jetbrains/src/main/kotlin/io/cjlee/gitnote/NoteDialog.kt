package io.cjlee.gitnote

import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.convertValue
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogWrapper
import com.intellij.ui.jcef.JBCefJSQuery
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Message
import io.cjlee.gitnote.jcef.GitNoteViewerWindow
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
    private val line: Int,
    private val onDispose: () -> Unit
) : DialogWrapper(true) {

    private val mapper = jacksonObjectMapper().registerModule(JavaTimeModule())
    private lateinit var window: GitNoteViewerWindow

    init {
        title = "Gitnote"
        setSize(500, 200)
        init()
    }

    override fun createCenterPanel(): JComponent {
        if (project == null) {
            throw IllegalStateException("project null")
        }
        val service = project.getService(JcefViewerWindowService::class.java)

        val protocolHandlers = mapOf(
            "messages/read" to object: MessageProtocolHandler {
                override fun handle(data: Any?): JBCefJSQuery.Response {
                    val messages = handler.readMessages(filePath, line)
                    val resp = mapper.writeValueAsString(messages)
                    return JBCefJSQuery.Response(resp)
                }
            },
            "messages/update" to object: MessageProtocolHandler {
                override fun handle(data: Any?): JBCefJSQuery.Response {
                    val message =  mapper.convertValue<Message>(data!!)
                    handler.update(filePath, message.line, message.message)
                    return JBCefJSQuery.Response("")
                }
            },
            "messages/delete" to object: MessageProtocolHandler {
                override fun handle(data: Any?): JBCefJSQuery.Response {
                    val message =  mapper.convertValue<Message>(data!!)
                    handler.delete(filePath, message.line)
                    return JBCefJSQuery.Response("")
                }
            },
            "messages/create" to object: MessageProtocolHandler {
                override fun handle(data: Any?): JBCefJSQuery.Response {
                    val message =  mapper.convertValue<Message>(data!!)
                    handler.add(filePath, message.line, message.message)
                    return JBCefJSQuery.Response("")
                }
            },
        )
        this.window = service.newWindow(protocolHandlers)

        return JPanel().apply {
            layout = GridLayout(0, 1)
            add(window.content)
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
