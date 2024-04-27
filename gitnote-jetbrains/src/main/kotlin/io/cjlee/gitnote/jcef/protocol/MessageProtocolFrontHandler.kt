package io.cjlee.gitnote.jcef.protocol

import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import java.util.function.Function
import com.intellij.ui.jcef.JBCefJSQuery.Response

class MessageProtocolFrontHandler(
    private val handlers: Map<String, MessageProtocolHandler>,
) : Function<String, Response> {
    private val mapper = jacksonObjectMapper()
    
    override fun apply(input: String): Response {
        println("received data from webview : $input")
        val protocol = mapper.readValue<MessageProtocol>(input)
        val handler = handlers[protocol.messageType]

//        return handler.handle(protocol.data) // TODO : Implement this
        return Response("Hello from IntelliJ")
    }
}

