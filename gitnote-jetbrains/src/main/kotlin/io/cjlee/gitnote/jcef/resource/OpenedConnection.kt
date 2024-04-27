package io.cjlee.gitnote.jcef.resource

import org.cef.callback.CefCallback
import org.cef.handler.CefLoadHandler
import org.cef.misc.IntRef
import org.cef.misc.StringRef
import org.cef.network.CefResponse
import java.io.IOException
import java.io.InputStream
import java.net.URLConnection

class OpenedConnection(private val connection: URLConnection) : ResourceHandlerState {
    private val inputStream: InputStream by lazy { connection.inputStream }

    override fun getResponseHeaders(
        cefResponse: CefResponse,
        responseLength: IntRef,
        redirectUrl: StringRef
    ) {
        try {
            val url = connection.url.toString()
            cefResponse.mimeType = when {
                url.contains("css") -> "text/css"
                url.contains("js") -> "text/javascript"
                url.contains("html") -> "text/html"
                else -> connection.contentType
            }
            responseLength.set(inputStream.available())
            cefResponse.status = 200
        } catch (e: IOException) {
            cefResponse.error = CefLoadHandler.ErrorCode.ERR_FILE_NOT_FOUND
            cefResponse.statusText = e.localizedMessage
            cefResponse.status = 404
        }
    }

    override fun readResponse(
        dataOut: ByteArray,
        designedBytesToRead: Int,
        bytesRead: IntRef,
        callback: CefCallback
    ): Boolean {
        val availableSize = inputStream.available()
        if (availableSize > 0) {
            val maxBytesToRead = minOf(availableSize, designedBytesToRead)
            val realNumberOfReadBytes = inputStream.read(dataOut, 0, maxBytesToRead)
            bytesRead.set(realNumberOfReadBytes)
            return true
        } else {
            inputStream.close()
            return false
        }
    }

    override fun close() {
        inputStream.close()
    }
}
