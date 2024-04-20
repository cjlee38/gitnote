package io.cjlee.gitnote

import com.intellij.openapi.editor.markup.GutterIconRenderer
import com.intellij.openapi.util.IconLoader
import javax.swing.Icon

class NoteIconGutterIconRenderer(
    private val message: String
): GutterIconRenderer() {
    override fun getIcon(): Icon = ICON

    override fun getTooltipText(): String = message

    override fun equals(other: Any?): Boolean = other is GutterIconRenderer && other.icon == this.icon

    override fun hashCode(): Int = icon.hashCode()

    companion object {
        val ICON = IconLoader.getIcon("/icons/ic_linemarkerprovider.svg", NoteIconGutterIconRenderer::class.java)
    }
}
