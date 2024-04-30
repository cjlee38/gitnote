package io.cjlee.gitnote.jcef

import com.intellij.openapi.project.Project
import com.intellij.openapi.util.Disposer
import com.intellij.ui.jcef.JBCefBrowser
import com.intellij.ui.jcef.JBCefBrowserBase
import com.intellij.ui.jcef.JBCefClient
import com.intellij.ui.jcef.JBCefJSQuery
import com.intellij.ui.jcef.executeJavaScriptAsync
import io.cjlee.gitnote.jcef.protocol.JcefInjectionLoadHandler
import io.cjlee.gitnote.jcef.protocol.MessageProtocolFrontHandler
import io.cjlee.gitnote.jcef.protocol.MessageProtocolHandler
import org.cef.CefApp
import org.cef.CefSettings
import org.cef.browser.CefBrowser
import org.cef.browser.CefFrame
import org.cef.handler.CefDisplayHandler
import javax.swing.JComponent


class GitNoteViewerWindow(private val project: Project, private val protocolHandlers: Map<String, MessageProtocolHandler>) {
    private val webView: JBCefBrowser = JBCefBrowser().apply {
        this.loadURL("http://gitnote/index.html")
        registerAppSchemeHandler()
        registerProtocolHandlers(this)
        jbCefClient.setProperty(JBCefClient.Properties.JS_QUERY_POOL_SIZE, 200)

        // TODO : Don't use Project as disposable in plugin code(Choosing a Disposable Parent)
        Disposer.register(project, this)
    }

    private fun registerProtocolHandlers(browser: JBCefBrowser) {
        val jsQuery = JBCefJSQuery.create((browser as JBCefBrowserBase))

        // inject query into javascript
        browser.jbCefClient.addLoadHandler(JcefInjectionLoadHandler(jsQuery), browser.cefBrowser)
        jsQuery.addHandler(MessageProtocolFrontHandler(browser, protocolHandlers))

        browser.jbCefClient.addDisplayHandler(JCefDebugDisplayHandler(), browser.cefBrowser) // for debugging
    }

    val content: JComponent
        get() = webView.component

    private fun registerAppSchemeHandler() {
        CefApp.getInstance()
            .registerSchemeHandlerFactory("http", "gitnote", JcefSchemeHandlerFactory())
    }
}

class JCefDebugDisplayHandler : CefDisplayHandler {
    override fun onAddressChange(browser: CefBrowser?, frame: CefFrame?, url: String?) {

    }

    override fun onTitleChange(browser: CefBrowser?, title: String?) {
    }

    override fun onTooltip(browser: CefBrowser?, text: String?): Boolean {
        return true
    }

    override fun onStatusMessage(browser: CefBrowser?, value: String?) {
    }

    override fun onConsoleMessage(
        browser: CefBrowser?,
        level: CefSettings.LogSeverity?,
        message: String?,
        source: String?,
        line: Int
    ): Boolean {
        println("Console message: $message (source: $source, line: $line)")
        return true
    }

    override fun onCursorChange(browser: CefBrowser?, cursorType: Int): Boolean {
        return true
    }
}
