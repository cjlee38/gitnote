package io.cjlee.gitnote.jcef.protocol

import com.intellij.ui.jcef.JBCefJSQuery

interface ProtocolHandler {
    fun handle(data: Any?): JBCefJSQuery.Response
}
