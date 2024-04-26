package io.cjlee.gitnote.toolWindow

import com.intellij.openapi.project.Project
import com.intellij.openapi.util.Disposer
import com.intellij.openapi.wm.ToolWindow
import com.intellij.openapi.wm.ToolWindowFactory
import com.intellij.ui.jcef.JBCefBrowser
import org.cef.CefApp
import org.cef.browser.CefBrowser
import org.cef.browser.CefFrame
import org.cef.callback.CefCallback
import org.cef.callback.CefSchemeHandlerFactory
import org.cef.handler.CefLoadHandler
import org.cef.handler.CefResourceHandler
import org.cef.misc.IntRef
import org.cef.misc.StringRef
import org.cef.network.CefRequest
import org.cef.network.CefResponse
import java.io.IOException
import java.io.InputStream
import java.net.URLConnection
import javax.swing.JComponent

class WindowFactory : ToolWindowFactory {

    override fun createToolWindowContent(project: Project, toolWindow: ToolWindow) {
        val component = toolWindow.component
        val catViewerWindow = project.getService(CatViewerWindowService::class.java).catViewerWindow
        component.parent.add(catViewerWindow.getContent())
    }
}

class CatViewerWindowService(val project: Project) {
    val catViewerWindow = CatViewerWindow(project)
}

class CatViewerWindow(private val project: Project) {
    private val webView: JBCefBrowser by lazy {
        val browser = JBCefBrowser()
        registerAppSchemeHandler()
        browser.loadURL("http://myapp/index.html")
        Disposer.register(project, browser)
        browser
    }

    fun getContent(): JComponent = webView.component

    private fun registerAppSchemeHandler() {
        CefApp.getInstance()
            .registerSchemeHandlerFactory(
                "http",
                "myapp",
                CustomSchemeHandlerFactory()
            )
    }
}

class CustomSchemeHandlerFactory: CefSchemeHandlerFactory {
    override fun create(
        cefBrowser: CefBrowser,
        cefFrame: CefFrame,
        s: String,
        cefRequest: CefRequest
    ): CefResourceHandler = CustomResourceHandler()
}

class CustomResourceHandler : CefResourceHandler {
    private var state: ResourceHandlerState = ClosedConnection
    override fun processRequest(
        cefRequest: CefRequest,
        cefCallback: CefCallback
    ): Boolean {
        val url = cefRequest.url ?: return false
        val pathToResource = url.replace("http://myapp", "webview/")
        val resource = this::class.java.classLoader.getResource(pathToResource)
        state = OpenedConnection(resource.openConnection())
        cefCallback.Continue()
        return true
    }

    override fun getResponseHeaders(
        cefResponse: CefResponse,
        responseLength: IntRef,
        redirectUrl: StringRef
    ) {
        state.getResponseHeaders(cefResponse, responseLength, redirectUrl)
    }

    override fun readResponse(
        dataOut: ByteArray,
        designedBytesToRead: Int,
        bytesRead: IntRef,
        callback: CefCallback
    ): Boolean {
        return state.readResponse(dataOut, designedBytesToRead, bytesRead, callback)
    }

    override fun cancel() {
        state.close()
        state = ClosedConnection
    }
}

sealed interface ResourceHandlerState {
    fun getResponseHeaders(
        cefResponse: CefResponse,
        responseLength: IntRef,
        redirectUrl: StringRef
    )

    fun readResponse(
        dataOut: ByteArray,
        designedBytesToRead: Int,
        bytesRead: IntRef,
        callback: CefCallback
    ): Boolean

    fun close() {
    }
}

class OpenedConnection(private val connection: URLConnection) : ResourceHandlerState {
    private val inputStream: InputStream by lazy { connection.inputStream }

    override fun getResponseHeaders(
        cefResponse: CefResponse,
        responseLength: IntRef,
        redirectUrl: StringRef
    ) {
        try {
            val url = connection.url.toString()
            cefResponse.mimeType = when {
                url.contains("css") -> "text/css"
                url.contains("js") -> "text/javascript"
                url.contains("html") -> "text/html"
                else -> connection.contentType
            }
            responseLength.set(inputStream.available())
            cefResponse.status = 200
        } catch (e: IOException) {
            cefResponse.error = CefLoadHandler.ErrorCode.ERR_FILE_NOT_FOUND
            cefResponse.statusText = e.localizedMessage
            cefResponse.status = 404
        }
    }

    override fun readResponse(
        dataOut: ByteArray,
        designedBytesToRead: Int,
        bytesRead: IntRef,
        callback: CefCallback
    ): Boolean {
        val availableSize = inputStream.available()
        if (availableSize > 0) {
            val maxBytesToRead = minOf(availableSize, designedBytesToRead)
            val realNumberOfReadBytes = inputStream.read(dataOut, 0, maxBytesToRead)
            bytesRead.set(realNumberOfReadBytes)
            return true
        } else {
            inputStream.close()
            return false
        }
    }

    override fun close() {
        inputStream.close()
    }
}

object ClosedConnection : ResourceHandlerState {
    override fun getResponseHeaders(
        cefResponse: CefResponse,
        responseLength: IntRef,
        redirectUrl: StringRef
    ) {
        cefResponse.status = 404
    }

    override fun readResponse(
        dataOut: ByteArray,
        designedBytesToRead: Int,
        bytesRead: IntRef,
        callback: CefCallback
    ): Boolean {
        return false
    }
}
