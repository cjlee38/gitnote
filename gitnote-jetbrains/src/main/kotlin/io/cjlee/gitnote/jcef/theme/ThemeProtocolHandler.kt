package io.cjlee.gitnote.jcef.theme

import com.intellij.openapi.editor.colors.EditorColorsManager
import com.intellij.ui.JBColor
import io.cjlee.gitnote.jcef.protocol.ProtocolHandler

class ThemeProtocolHandler : ProtocolHandler {
    override fun handle(data: Any?): ProtocolHandler.Response {
        val globalScheme = EditorColorsManager.getInstance().globalScheme
        val editorBackground = globalScheme.defaultBackground
        return ProtocolHandler.Response(
            mapOf(
                "editorBackground" to editorBackground.rgb,
                "background" to JBColor.background().rgb,
                "text" to JBColor.foreground().rgb,
            )
        )
    }
}
