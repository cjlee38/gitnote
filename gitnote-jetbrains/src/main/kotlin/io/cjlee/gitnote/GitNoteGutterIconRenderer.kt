package io.cjlee.gitnote

import com.intellij.openapi.Disposable
import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.editor.Document
import com.intellij.openapi.editor.markup.GutterDraggableObject
import com.intellij.openapi.editor.markup.GutterIconRenderer
import com.intellij.openapi.util.IconLoader
import com.intellij.util.IconUtil
import com.intellij.util.ui.EmptyIcon
import com.intellij.util.ui.ImageUtil
import io.cjlee.gitnote.core.Message
import io.cjlee.gitnote.jcef.protocol.ProtocolHandler
import java.awt.AlphaComposite
import java.awt.image.BufferedImage
import javax.swing.Icon
import javax.swing.ImageIcon

class GitNoteGutterIconRenderer(
    val line: Int,
    val lineMessages: List<Message>,
    private val protocolHandlers: Map<String, ProtocolHandler>,
    var visible: Boolean,
    val document: Document
) : GutterIconRenderer(), Disposable {
    val hasMessage: Boolean = lineMessages.isNotEmpty()

    override fun getIcon(): Icon {
        return when {
            visible && hasMessage -> ICON
            visible && !hasMessage -> TRANSPARENT
            else -> EmptyIcon.ICON_16
        }
    }

    override fun getTooltipText(): String = lineMessages.lastOrNull()?.message ?: "Add a new note"

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

    override fun equals(other: Any?): Boolean = other is GutterIconRenderer && other.icon == this.icon

    override fun hashCode(): Int = icon.hashCode()

    override fun dispose() {
        // TODO : dispose
    }

    override fun getDraggableObject(): GutterDraggableObject? {
        // TODO : drag & drop
        return super.getDraggableObject()
    }

    override fun toString(): String {
        return "GitNoteGutterIconRenderer(line=$line, lineMessages=$lineMessages, protocolHandlers=$protocolHandlers, visible=$visible, document=$document, hasMessage=$hasMessage)"
    }


    companion object IconRenderer {
        private val ICON = IconLoader.getIcon("/icons/icon.png", GitNoteGutterIconRenderer::class.java)
            .let { IconUtil.scale(it, null, (13.0 / it.iconWidth).toFloat()) }
        private val TRANSPARENT = makeIconTransparent(ICON, 0.5f)
            .let { IconUtil.scale(it, null, (13.0 / it.iconWidth).toFloat()) }

        private fun makeIconTransparent(icon: Icon, alpha: Float): Icon {
            val bufferedImage = ImageUtil.createImage(icon.iconWidth, icon.iconHeight, BufferedImage.TYPE_INT_ARGB)
            val g2d = bufferedImage.createGraphics()
            g2d.composite = AlphaComposite.getInstance(AlphaComposite.SRC_OVER, alpha)
            icon.paintIcon(null, g2d, 0, 0)
            g2d.dispose()

            return ImageIcon(bufferedImage)
        }
    }
}
