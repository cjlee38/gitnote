package io.cjlee.gitnote

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.editor.markup.GutterIconRenderer
import com.intellij.openapi.util.IconLoader
import com.intellij.util.IconUtil
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Message
import javax.swing.Icon

// TODO : show hand cursor when hover
class NoteIconGutterIconRenderer(
    private val filePath: String,
    messages: List<Message>,
    private val handler: CoreHandler
) : GutterIconRenderer() {
    private val messages = messages.sortedByDescending { it.createdAt }

    override fun getIcon(): Icon = ICON

    override fun getTooltipText(): String = messages.last().message

    override fun equals(other: Any?): Boolean = other is GutterIconRenderer && other.icon == this.icon

    override fun hashCode(): Int = icon.hashCode()

    companion object {
        val ICON = IconLoader.getIcon("/icons/icon.png", NoteIconGutterIconRenderer::class.java)
            .let { IconUtil.scale(it, null, (13.0 / it.iconWidth).toFloat()) }
    }

    override fun getClickAction(): AnAction {
        return object : AnAction() {
            override fun actionPerformed(e: AnActionEvent) {
                NoteDialog(e.project, handler, messages).show()
            }
        }
    }
}
