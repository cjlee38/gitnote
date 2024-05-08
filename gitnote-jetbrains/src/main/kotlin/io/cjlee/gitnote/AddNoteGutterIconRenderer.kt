package io.cjlee.gitnote

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.util.IconUtil
import com.intellij.util.ui.ImageUtil
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Message
import java.awt.AlphaComposite
import java.awt.image.BufferedImage
import javax.swing.Icon
import javax.swing.ImageIcon

class AddNoteGutterIconRenderer(
    filePath: String,
    handler: CoreHandler,
    line: Int,
    onDispose: () -> Unit
): NoteGutterIconRenderer(filePath, handler, line, onDispose) {
    override fun getIcon(): Icon {
        return makeIconTransparent(ICON, 0.5f).let { IconUtil.scale(it, null, (13.0 / it.iconWidth).toFloat())}
    }

    private fun makeIconTransparent(icon: Icon, alpha: Float): Icon {
        // Convert Icon to BufferedImage
        val bufferedImage = ImageUtil.createImage(icon.iconWidth, icon.iconHeight, BufferedImage.TYPE_INT_ARGB)
        val g2d = bufferedImage.createGraphics()

        // Set the transparency level
        g2d.composite = AlphaComposite.getInstance(AlphaComposite.SRC_OVER, alpha)

        // Draw the icon on the BufferedImage
        icon.paintIcon(null, g2d, 0, 0)
        g2d.dispose()

        return ImageIcon(bufferedImage)
    }
}

