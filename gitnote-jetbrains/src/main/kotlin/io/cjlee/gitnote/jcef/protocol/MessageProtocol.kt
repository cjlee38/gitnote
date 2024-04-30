package io.cjlee.gitnote.jcef.protocol

data class MessageProtocol(
    val messageType: String,
    val messageId: String,
    val data: Any?,
)
