package io.cjlee.gitnote.jcef.theme

import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.intellij.openapi.editor.colors.EditorColors
import com.intellij.openapi.editor.colors.EditorColorsManager
import com.intellij.ui.JBColor
import com.intellij.ui.jcef.JBCefJSQuery
import io.cjlee.gitnote.jcef.protocol.MessageProtocolHandler

class ThemeMessageProtocolHandler : MessageProtocolHandler {
    private val mapper = jacksonObjectMapper()

    override fun handle(data: Any?): JBCefJSQuery.Response {
        val globalScheme = EditorColorsManager.getInstance().globalScheme
        val editorBackground = globalScheme.defaultBackground
        val defaultForeground = globalScheme.defaultForeground
        val theme = mapOf(
            "editorBackground" to editorBackground.rgb,
            "background" to JBColor.background().rgb,
            "text" to JBColor.foreground().rgb,
        )

        return JBCefJSQuery.Response(mapper.writeValueAsString(theme))
    }
}
