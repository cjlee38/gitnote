package io.cjlee.gitnote.jcef.protocol

import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import com.intellij.ui.jcef.JBCefBrowser
import com.intellij.ui.jcef.JBCefJSQuery.Response
import com.intellij.ui.jcef.executeJavaScriptAsync
import java.util.function.Function

class MessageProtocolFrontHandler(
    private val webView: JBCefBrowser,
    handlers: Map<String, MessageProtocolHandler>,
) : Function<String, Response> {
    private val mapper = jacksonObjectMapper()
    private val handlers = handlers.toMutableMap()

    override fun apply(input: String): Response {
        println("received data from webview : $input")
        val protocol = mapper.readValue<MessageProtocol>(input)
        val handler = handlers[protocol.messageType] ?: object : MessageProtocolHandler {
            override fun handle(data: Any?): Response {
                return Response("No handler for ${protocol.messageType}")
            }
        }
        val response = handler.handle(protocol.data)
        // TODO : For now, manually response to webview by executing javascript
        sendToWebView(protocol.messageType, response.response(), protocol.messageId)
        return Response("")
    }

    fun addHandler(messageType: String, handler: MessageProtocolHandler) {
        handlers[messageType] = handler
    }

    private fun sendToWebView(messageType: String, data: Any?, messageId: String) {
        val snippet = buildJavascriptMessageSnippet(messageType, data, messageId)
        println("response to webview : $snippet")
        webView.executeJavaScriptAsync(snippet)
    }

    private fun buildJavascriptMessageSnippet(messageType: String, data: Any?, messageId: String): String {
        return """window.postMessage({type : '$messageType', data : '$data', messageId : '$messageId'})"""
    }
}
