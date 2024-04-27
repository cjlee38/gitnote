package io.cjlee.gitnote.jcef.protocol

data class MessageProtocol(
    val messageType: String,
    val data: Any?,
)
