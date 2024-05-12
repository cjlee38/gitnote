package io.cjlee.gitnote.jcef.theme

import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.intellij.openapi.editor.colors.EditorColorsManager
import com.intellij.ui.JBColor
import io.cjlee.gitnote.jcef.protocol.ProtocolHandler

class ThemeProtocolHandler : ProtocolHandler {
    private val mapper = jacksonObjectMapper()

    override fun handle(data: Any?): Any {
        val globalScheme = EditorColorsManager.getInstance().globalScheme
        val editorBackground = globalScheme.defaultBackground
        return mapOf(
            "editorBackground" to editorBackground.rgb,
            "background" to JBColor.background().rgb,
            "text" to JBColor.foreground().rgb,
        )
    }
}
