package io.cjlee.gitnote.jcef

import com.intellij.openapi.project.Project
import com.intellij.openapi.util.Disposer
import com.intellij.ui.jcef.JBCefBrowser
import org.cef.CefApp
import javax.swing.JComponent

class GitNoteViewerWindow(private val project: Project) {
    private val webView: JBCefBrowser by lazy {
        val browser = JBCefBrowser()
        registerAppSchemeHandler()
        browser.loadURL("http://gitnote/index.html")
        // TODO : Don't use Project as disposable in plugin code(Choosing a Disposable Parent)
        Disposer.register(project, browser)
        browser
    }

    val content: JComponent
        get() = webView.component

    private fun registerAppSchemeHandler() {
        CefApp.getInstance()
            .registerSchemeHandlerFactory("http", "gitnote", JcefSchemeHandlerFactory())
    }
}
