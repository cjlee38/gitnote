package io.cjlee.gitnote.jcef.protocol

import com.intellij.ui.jcef.JBCefJSQuery
import org.cef.browser.CefBrowser
import org.cef.handler.CefLoadHandlerAdapter

/**
 * This class is responsible for injecting the JavaScript bridge into the loaded page.
 */
class JcefInjectionLoadHandler(
    private val jsQuery: JBCefJSQuery,
): CefLoadHandlerAdapter() {
    override fun onLoadingStateChange(
        browser: CefBrowser?,
        isLoading: Boolean,
        canGoBack: Boolean,
        canGoForward: Boolean
    ) {
        if (!isLoading) {
            // The page has finished loading
            injectJavascriptBridge(browser, jsQuery)
        }
    }

    private fun injectJavascriptBridge(browser: CefBrowser?, jsQuery: JBCefJSQuery) {
        // TODO : might need to handle response/error
        val script = """window.sendMessageToIde = function(type, data, id) {
                const msg = JSON.stringify({type, data, id});
                ${jsQuery.inject("msg")}
            }""".trimIndent()

        browser?.executeJavaScript(script, browser.url, 0)
    }
}
