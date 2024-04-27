package io.cjlee.gitnote.jcef.resource

import org.cef.callback.CefCallback
import org.cef.handler.CefResourceHandler
import org.cef.misc.IntRef
import org.cef.misc.StringRef
import org.cef.network.CefRequest
import org.cef.network.CefResponse

class JcefResourceHandler : CefResourceHandler {
    private var state: ResourceHandlerState = ClosedConnection

    override fun processRequest(
        cefRequest: CefRequest,
        cefCallback: CefCallback
    ): Boolean {
        val url = cefRequest.url ?: return false
        val pathToResource = url.replace("http://gitnote/", "webview/")
        val resource = this::class.java.classLoader.getResource(pathToResource) ?: return false
        state = OpenedConnection(resource.openConnection())
        cefCallback.Continue()
        return true
    }

    override fun getResponseHeaders(
        cefResponse: CefResponse,
        responseLength: IntRef,
        redirectUrl: StringRef
    ) {
        state.getResponseHeaders(cefResponse, responseLength, redirectUrl)
    }

    override fun readResponse(
        dataOut: ByteArray,
        designedBytesToRead: Int,
        bytesRead: IntRef,
        callback: CefCallback
    ): Boolean {
        return state.readResponse(dataOut, designedBytesToRead, bytesRead, callback)
    }

    override fun cancel() {
        state.close()
        state = ClosedConnection
    }
}
