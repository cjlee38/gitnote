package io.cjlee.gitnote

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.editor.markup.GutterDraggableObject
import com.intellij.openapi.editor.markup.GutterIconRenderer
import com.intellij.openapi.util.IconLoader
import com.intellij.util.IconUtil
import io.cjlee.gitnote.core.Message
import io.cjlee.gitnote.jcef.protocol.ProtocolHandler
import javax.swing.Icon

open class GitNoteGutterIconRenderer(
    private val messages: List<Message>,
    private val protocolHandlers: Map<String, ProtocolHandler>
) : GutterIconRenderer() {

    override fun getIcon(): Icon = ICON

    override fun getTooltipText(): String = messages.last().message

    override fun equals(other: Any?): Boolean = other is GutterIconRenderer && other.icon == this.icon

    override fun hashCode(): Int = icon.hashCode()

    override fun isNavigateAction(): Boolean {
        return true
    }

    override fun getAlignment(): Alignment {
        return Alignment.RIGHT
    }

    override fun getClickAction(): AnAction {
        return object : AnAction() {
            override fun actionPerformed(e: AnActionEvent) {
                val gitNoteDialog = GitNoteDialog(e.project, protocolHandlers)
                gitNoteDialog.show()
            }
        }
    }

    open val line: Int
        get() = messages.last().line

    override fun getDraggableObject(): GutterDraggableObject? {
        // TODO : drag & drop
        return super.getDraggableObject()
    }

    companion object {
        val ICON = IconLoader.getIcon("/icons/icon.png", GitNoteGutterIconRenderer::class.java)
            .let { IconUtil.scale(it, null, (13.0 / it.iconWidth).toFloat()) }
    }
}
