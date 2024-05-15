package io.cjlee.gitnote.jcef.protocol

interface ProtocolHandler {
    fun handle(data: Any?): Response

    data class Response(
        val data: Any? = null,
        val error: String? = null,
    )
}
