package io.cjlee.gitnote.jcef

import com.intellij.openapi.project.Project
import com.intellij.openapi.util.Disposer
import com.intellij.ui.jcef.JBCefBrowser
import com.intellij.ui.jcef.JBCefBrowserBase
import com.intellij.ui.jcef.JBCefClient
import com.intellij.ui.jcef.JBCefJSQuery
import io.cjlee.gitnote.jcef.protocol.JcefInjectionLoadHandler
import io.cjlee.gitnote.jcef.protocol.ProtocolFrontHandler
import io.cjlee.gitnote.jcef.protocol.ProtocolHandler
import io.cjlee.gitnote.jcef.theme.ThemeProtocolHandler
import org.cef.CefApp
import org.cef.CefSettings
import org.cef.browser.CefBrowser
import org.cef.browser.CefFrame
import org.cef.handler.CefDisplayHandler
import javax.swing.JComponent


class GitNoteViewerWindow(private val project: Project, private val protocolHandlers: Map<String, ProtocolHandler>) {
    private val webView: JBCefBrowser = JBCefBrowser().apply {
        val isDevelopment = System.getProperty("gitnote.developmentPhase", "false").toBoolean()
        if (isDevelopment) this.loadURL("http://localhost:3000/index.html")
        else this.loadURL("http://gitnote/index.html")

        registerAppSchemeHandler()
        registerProtocolHandlers(this)
        jbCefClient.setProperty(JBCefClient.Properties.JS_QUERY_POOL_SIZE, 200)
        this.setProperty(JBCefBrowserBase.Properties.NO_CONTEXT_MENU, true)

        // TODO : Don't use Project as disposable in plugin code(Choosing a Disposable Parent)
        Disposer.register(project, this)
    }

    private fun registerProtocolHandlers(browser: JBCefBrowser) {
        val jsQuery = JBCefJSQuery.create((browser as JBCefBrowserBase))

        // inject query into javascript
        browser.jbCefClient.addLoadHandler(JcefInjectionLoadHandler(jsQuery), browser.cefBrowser)
        val frontHandler = ProtocolFrontHandler(browser, protocolHandlers)
        frontHandler.addHandler("theme", ThemeProtocolHandler())
        jsQuery.addHandler(frontHandler)

        browser.jbCefClient.addDisplayHandler(JCefDebugDisplayHandler(), browser.cefBrowser) // for debugging
    }

    val content: JComponent
        get() = webView.component

    private fun registerAppSchemeHandler() {
        CefApp.getInstance()
            .registerSchemeHandlerFactory("http", "gitnote", JcefSchemeHandlerFactory())
    }

    fun dispose() {
        webView.dispose()
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
