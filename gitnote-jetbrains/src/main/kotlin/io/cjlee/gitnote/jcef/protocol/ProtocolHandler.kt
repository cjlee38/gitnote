package io.cjlee.gitnote.jcef.protocol

interface ProtocolHandler {
    fun handle(data: Any?): Any?
}
