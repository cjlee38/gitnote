package io.cjlee.gitnote

import com.intellij.openapi.ui.DialogWrapper
import com.intellij.openapi.editor.colors.EditorColorsManager
import com.intellij.openapi.editor.colors.EditorFontType
import com.intellij.ui.JBColor
import com.intellij.ui.components.JBScrollPane
import io.cjlee.gitnote.core.CoreHandler
import io.cjlee.gitnote.core.Message
import javax.swing.*
import java.awt.*
import javax.swing.border.LineBorder

class NoteDialog(private val handler: CoreHandler, private val message: Message) : DialogWrapper(true) {
    private val containerPanel = JPanel()
    private lateinit var inputTextField: JTextField

    init {
        title = "Ginote"
        setSize(800, 600)
        init()
    }

    // TODO : cocnnect to handler
    // TODO : show message from gitnote
    override fun createCenterPanel(): JComponent {
        val fixedHeight = 50
        val fixedWidth = 700

        val firstElementPanel = createElementPanel("First text element", fixedWidth)
        val secondElementPanel = createElementPanel("Second text element", fixedWidth)
        val thirdElementPanel = createElementPanel("Third text element", fixedWidth)

        containerPanel.layout = BoxLayout(containerPanel, BoxLayout.Y_AXIS) // Vertical alignment
        containerPanel.add(firstElementPanel)
        containerPanel.add(secondElementPanel)
        containerPanel.add(thirdElementPanel)

        val scrollPane = JBScrollPane(containerPanel).apply {
            verticalScrollBarPolicy = JBScrollPane.VERTICAL_SCROLLBAR_ALWAYS
            horizontalScrollBarPolicy = JBScrollPane.HORIZONTAL_SCROLLBAR_NEVER
        }

        inputTextField = JTextField(20)
        inputTextField.preferredSize = Dimension(fixedWidth, fixedHeight)

        val mainPanel = JPanel(BorderLayout())
        mainPanel.add(scrollPane, BorderLayout.CENTER)
        mainPanel.add(inputTextField, BorderLayout.SOUTH)

        return mainPanel
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

        return JPanel().apply {
            layout = FlowLayout(FlowLayout.LEFT)
            border = LineBorder(JBColor.BLACK, 1, true)

            val label = JLabel(text)
            label.font = editorFont

            val fm = label.getFontMetrics(editorFont)

            val textWidth = fm.stringWidth(text)
            val lines = Math.ceil(textWidth.toDouble() / width).toInt()
            val lineHeight = fm.height

            val calculatedHeight = lines * lineHeight

            preferredSize = Dimension(width, calculatedHeight)

            add(label)
        }
    }
}

