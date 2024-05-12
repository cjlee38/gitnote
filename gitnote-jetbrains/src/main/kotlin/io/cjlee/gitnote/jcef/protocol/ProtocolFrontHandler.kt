package io.cjlee.gitnote.jcef.protocol

import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import com.intellij.ui.jcef.JBCefBrowser
import com.intellij.ui.jcef.JBCefJSQuery.Response
import com.intellij.ui.jcef.executeJavaScriptAsync
import java.util.function.Function

class ProtocolFrontHandler(
    private val webView: JBCefBrowser,
    handlers: Map<String, ProtocolHandler>,
) : Function<String, Response> {
    private val mapper = jacksonObjectMapper()
    private val handlers = handlers.toMutableMap()

    override fun apply(input: String): Response {
        println("received data from webview : $input")
        val protocol = mapper.readValue<Protocol>(input)
        val handler = handlers[protocol.type] ?: object : ProtocolHandler {
            override fun handle(data: Any?): String {
                return "No handler for ${protocol.type}"
            }
        }
        val payload = handler.handle(protocol.payload)
        // TODO : For now, manually response to webview by executing javascript
        sendToWebView(protocol.type, payload, protocol.id)
        return Response("")
    }

    fun addHandler(type: String, handler: ProtocolHandler) {
        handlers[type] = handler
    }

    private fun sendToWebView(type: String, payload: Any?, id: String) {
        val protocol = Protocol(type, id, payload)
        val serialized = mapper.writeValueAsString(protocol)
        val snippet = buildJavascriptMessageSnippet(serialized)
        println("response to webview : $snippet")
        webView.executeJavaScriptAsync(snippet)
    }

    private fun buildJavascriptMessageSnippet(protocolData: String): String {
        return """window.postMessage($protocolData)"""
    }
}
