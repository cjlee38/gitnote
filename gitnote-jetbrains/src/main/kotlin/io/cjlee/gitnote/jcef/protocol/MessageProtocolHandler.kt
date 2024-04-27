package io.cjlee.gitnote.jcef.protocol

import com.intellij.ui.jcef.JBCefJSQuery

sealed interface MessageProtocolHandler {
    fun handle(data: Any?): JBCefJSQuery.Response
}
