package io.cjlee.gitnote

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.editor.markup.GutterIconRenderer
import com.intellij.openapi.ui.Messages
import com.intellij.openapi.ui.popup.JBPopupFactory
import com.intellij.openapi.util.IconLoader
import com.intellij.ui.components.JBPanel
import com.intellij.ui.components.JBTextField
import com.intellij.util.IconUtil
import com.intellij.util.ui.JBUI
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Message
import java.awt.event.MouseAdapter
import java.awt.event.MouseEvent
import javax.swing.BoxLayout
import javax.swing.Icon
import javax.swing.JButton
import javax.swing.JDialog
import javax.swing.JLabel
import javax.swing.WindowConstants

class NoteIconGutterIconRenderer(
    private val filePath: String,
    private val message: Message,
    private val handler: CoreHandler
) : GutterIconRenderer() {
    override fun getIcon(): Icon = ICON

    override fun getTooltipText(): String = message.message

    override fun equals(other: Any?): Boolean = other is GutterIconRenderer && other.icon == this.icon

    override fun hashCode(): Int = icon.hashCode()

    companion object {
        val ICON = IconLoader.getIcon("/icons/icon.png", NoteIconGutterIconRenderer::class.java)
            .let { IconUtil.scale(it, null, (13.0 / it.iconWidth).toFloat()) }
    }

    override fun getClickAction(): AnAction? {
        return object : AnAction() {
            override fun actionPerformed(e: AnActionEvent) {
                val textField = JBTextField(message.message).apply {
                    isEditable = false
                }

                val okButton = JButton("OK").apply {
                    addActionListener {
                        handler.update(filePath, message.startLine, message.endLine, textField.text)
                        textField.isEditable = false
                    }
                    isVisible = false
                }

                val cancelButton = JButton("CANCEL").apply {
                    addActionListener {
                        textField.isEditable = false
                        okButton.isVisible = false
                        this.isVisible = false
                    }
                    isVisible = false
                }

                textField.addMouseListener(object : MouseAdapter() {
                    override fun mouseClicked(e: MouseEvent) {
                        textField.isEditable = true
                        okButton.isVisible = true
                        cancelButton.isVisible = true
                    }
                })

                val panel = JBPanel<JBPanel<*>>().apply {
                    layout = BoxLayout(this, BoxLayout.Y_AXIS)
                    border = JBUI.Borders.empty(10)
                    add(textField)
                    add(JLabel(" "))  // Spacer
                    add(okButton)
                    add(cancelButton)
                }

                val dialog = JDialog().apply {
                    title = "Edit Message"
                    contentPane = panel
                    defaultCloseOperation = WindowConstants.DISPOSE_ON_CLOSE
                    pack()
                    setLocationRelativeTo(null)  // Center the dialog
                    isVisible = true
                }
            }
        }

//        return object : AnAction() {
//            override fun actionPerformed(e: AnActionEvent) {
//                val panel = JBPanel<JBPanel<*>>().apply {
//                    layout = BoxLayout(this, BoxLayout.Y_AXIS)
//                    border = JBUI.Borders.empty(10)
//                }
//
//                val textField = JBTextField(message.message).apply {
//                    isEditable = false
//                }
//
//                val okButton = JButton("OK").apply {
//                    addActionListener {
//                        handler.update(filePath, message.startLine, message.endLine, textField.text)
//                        textField.isEditable = false
//                    }
//                    isVisible = false
//                }
//
//                val cancelButton = JButton("CANCEL").apply {
//                    addActionListener {
//                        textField.isEditable = false
//                        okButton.isVisible = false
//                        this.isVisible = false
//                    }
//                    isVisible = false
//                }
//
//                textField.addMouseListener(object : MouseAdapter() {
//                    override fun mouseClicked(e: MouseEvent) {
//                        textField.isEditable = true
//                        okButton.isVisible = true
//                        cancelButton.isVisible = true
//                    }
//                })
//
//                panel.add(textField)
//                panel.add(JLabel(" "))  // Spacer
//                panel.add(okButton)
//                panel.add(cancelButton)
//
//                JBPopupFactory.getInstance()
//                    .createComponentPopupBuilder(panel, null)
//                    .createPopup()
//                    .showInBestPositionFor(e.dataContext)
//            }
//        }
    }
}
