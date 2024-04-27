package io.cjlee.gitnote

import com.intellij.icons.AllIcons
import com.intellij.openapi.editor.colors.EditorColorsManager
import com.intellij.openapi.editor.colors.EditorFontType
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogWrapper
import com.intellij.ui.JBColor
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Message
import io.cjlee.gitnote.jcef.CatViewerWindowService
import java.awt.BorderLayout
import java.awt.Color
import java.awt.Dimension
import java.awt.FlowLayout
import java.time.LocalDateTime
import javax.swing.Action
import javax.swing.BoxLayout
import javax.swing.JButton
import javax.swing.JComponent
import javax.swing.JLabel
import javax.swing.JPanel
import javax.swing.JTextArea
import javax.swing.JTextField
import javax.swing.border.LineBorder

class NoteDialog(
    private val project: Project?,
    private val handler: CoreHandler,
    private val messages: List<Message>
) : DialogWrapper(true) {
    private val containerPanel = JPanel()
    private lateinit var inputTextField: JTextField

    init {
        title = "Gitnote"
        setSize(800, 600)
        init()
    }

    // TODO : connect to handler
    // TODO : convert into kotlin DSL
    // TODO : scroll missing
    // TODO : show icons & LocalDateTime when hover
    // TODO : fixed size for each row
    override fun createCenterPanel(): JComponent {
        if (project == null) {
            throw IllegalStateException("projec t null ")
        }
        val window = project.getService(CatViewerWindowService::class.java).catViewerWindow
        return JPanel().apply {
            add(window.content)
        }
    }

    override fun createActions(): Array<Action> {
        return arrayOf(okAction, cancelAction)
    }

    override fun doOKAction() {
        val userInput = inputTextField.text
        if (userInput.isNotEmpty()) {
            val fixedWidth = 700

            val newElementPanel = createElementPanel(userInput, fixedWidth)
            containerPanel.add(newElementPanel)
            containerPanel.revalidate()
            inputTextField.text = ""
        }
    }

    private fun createElementPanel(text: String, width: Int): JPanel {
        val editorFont = EditorColorsManager.getInstance().globalScheme.getFont(EditorFontType.PLAIN)

        val editIcon = JButton(AllIcons.Actions.EditSource).apply {
            preferredSize = Dimension(24, 24)
            background = Color(0, 0, 0, 0)
            isVisible = true
        }

        val removeIcon = JButton(AllIcons.Diff.Remove).apply {
            preferredSize = Dimension(24, 24)
            background = Color(0, 0, 0, 0)
            isVisible = true
        }

        val dateLabel = JLabel(LocalDateTime.now().toString()).apply {
            font = editorFont
            horizontalAlignment = JLabel.RIGHT
            background = Color(0, 0, 0, 0)
            isVisible = false
        }

//        val listener = object : MouseAdapter() {
//            override fun mouseEntered(e: MouseEvent) {
//                editIcon.isVisible = true
//                removeIcon.isVisible = true
////                dateLabel.isVisible = true
//                repaint()
//            }
//
//            override fun mouseExited(e: MouseEvent) {
//                editIcon.isVisible = false
//                removeIcon.isVisible = false
////                dateLabel.isVisible = false
//                repaint()
//            }
//        }

        val textArea = JTextArea(text).apply {
            isEditable = false
            lineWrap = true
            wrapStyleWord = true
            font = editorFont
//            isOpaque = false // Make transparent to mouse events
//            addMouseListener(listener)
        }

        val rightPanel = JPanel()
        rightPanel.layout = BoxLayout(rightPanel, BoxLayout.LINE_AXIS)
        rightPanel.add(editIcon)
        rightPanel.add(removeIcon)
        val leftPanel = JPanel()
        leftPanel.setLayout(FlowLayout(FlowLayout.LEFT));
        leftPanel.add(dateLabel)

        rightPanel.add(leftPanel)
//        val topPanel = JPanel(BoxLayout(rightPanel, BoxLayout.LINE_AXIS)).apply {
//            add(editIcon, FlowLayout.RIGHT)
//            add(removeIcon, FlowLayout.RIGHT)
//            add(dateLabel, FlowLayout.LEFT)
//            isOpaque = false // Make panel transparent to mouse events
//            addMouseListener(listener)
//        }

//        val bottomRightPanel = JPanel(FlowLayout(FlowLayout.RIGHT)).apply {
//            add(dateLabel)
//            isOpaque = false
//            addMouseListener(listener)
//        }


        return JPanel(BorderLayout()).apply {
            border = LineBorder(JBColor.BLACK, 1, true)
            preferredSize = Dimension(width, textArea.preferredSize.height)
            isOpaque = false // Ensure the main panel is transparent to mouse events

            add(rightPanel, BorderLayout.NORTH)
            add(textArea, BorderLayout.CENTER)
//            add(bottomRightPanel, BorderLayout.SOUTH)
            // Hover event listener
//            addMouseListener(listener)
        }
    }
}
