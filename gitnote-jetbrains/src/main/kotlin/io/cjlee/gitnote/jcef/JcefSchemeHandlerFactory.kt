package io.cjlee.gitnote.jcef

import io.cjlee.gitnote.jcef.resource.JcefResourceHandler
import org.cef.browser.CefBrowser
import org.cef.browser.CefFrame
import org.cef.callback.CefSchemeHandlerFactory
import org.cef.handler.CefResourceHandler
import org.cef.network.CefRequest

class JcefSchemeHandlerFactory: CefSchemeHandlerFactory {
    override fun create(
        cefBrowser: CefBrowser,
        cefFrame: CefFrame,
        s: String,
        cefRequest: CefRequest
    ): CefResourceHandler = JcefResourceHandler()
}
