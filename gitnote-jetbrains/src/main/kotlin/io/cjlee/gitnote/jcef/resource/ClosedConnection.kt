package io.cjlee.gitnote.jcef.resource

import org.cef.callback.CefCallback
import org.cef.misc.IntRef
import org.cef.misc.StringRef
import org.cef.network.CefResponse

data object ClosedConnection : ResourceHandlerState {
    override fun getResponseHeaders(
        cefResponse: CefResponse,
        responseLength: IntRef,
        redirectUrl: StringRef
    ) {
        cefResponse.status = 404
    }

    override fun readResponse(
        dataOut: ByteArray,
        designedBytesToRead: Int,
        bytesRead: IntRef,
        callback: CefCallback
    ): Boolean {
        return false
    }
}
