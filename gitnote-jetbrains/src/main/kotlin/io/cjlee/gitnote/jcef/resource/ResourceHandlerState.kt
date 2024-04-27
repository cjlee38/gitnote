package io.cjlee.gitnote.jcef.resource

import org.cef.callback.CefCallback
import org.cef.misc.IntRef
import org.cef.misc.StringRef
import org.cef.network.CefResponse

sealed interface ResourceHandlerState {
    fun getResponseHeaders(
        cefResponse: CefResponse,
        responseLength: IntRef,
        redirectUrl: StringRef
    )

    fun readResponse(
        dataOut: ByteArray,
        designedBytesToRead: Int,
        bytesRead: IntRef,
        callback: CefCallback
    ): Boolean

    fun close() {
    }
}

