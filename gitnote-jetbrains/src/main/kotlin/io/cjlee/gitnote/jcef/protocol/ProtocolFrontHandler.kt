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
            override fun handle(data: Any?): Response {
                return Response("No handler for ${protocol.type}")
            }
        }
        val response = handler.handle(protocol.data)
        // TODO : For now, manually response to webview by executing javascript
        sendToWebView(protocol.type, response.response(), protocol.id)
        return Response("")
    }

    fun addHandler(type: String, handler: ProtocolHandler) {
        handlers[type] = handler
    }

    private fun sendToWebView(type: String, data: Any?, id: String) {
        val snippet = buildJavascriptMessageSnippet(type, data, id)
        println("response to webview : $snippet")
        webView.executeJavaScriptAsync(snippet)
    }

    private fun buildJavascriptMessageSnippet(type: String, data: Any?, id: String): String {
        return """window.postMessage({type : '$type', data : '$data', id : '$id'})"""
    }
}
