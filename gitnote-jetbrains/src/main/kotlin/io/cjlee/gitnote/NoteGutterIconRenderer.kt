package io.cjlee.gitnote

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.editor.markup.GutterIconRenderer
import com.intellij.openapi.util.IconLoader
import com.intellij.util.IconUtil
import io.cjlee.gitnote.core.CoreHandler
import javax.swing.Icon

open class NoteGutterIconRenderer(
    private val filePath: String,
    private val handler: CoreHandler,
    private val line: Int,
    private val onDispose: () -> Unit
) : GutterIconRenderer() {

    override fun getIcon(): Icon = ICON

    override fun getTooltipText(): String = handler.read(filePath)?.messages?.find { it.line == line }?.message ?: ""

    override fun equals(other: Any?): Boolean = other is GutterIconRenderer && other.icon == this.icon

    override fun hashCode(): Int = icon.hashCode()

    override fun isNavigateAction(): Boolean {
        return true
    }

    override fun getClickAction(): AnAction {
        return object : AnAction() {
            override fun actionPerformed(e: AnActionEvent) {
                val noteDialog = NoteDialog(e.project, filePath, handler, line, onDispose)
                noteDialog.show()
            }
        }
    }

    companion object {
        val ICON = IconLoader.getIcon("/icons/icon.png", NoteGutterIconRenderer::class.java)
            .let { IconUtil.scale(it, null, (13.0 / it.iconWidth).toFloat()) }
    }
}
