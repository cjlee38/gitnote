package io.cjlee.gitnote

import com.intellij.openapi.editor.Editor
import com.intellij.openapi.editor.markup.LineMarkerRenderer
import com.intellij.ui.JBColor
import java.awt.Graphics
import java.awt.Rectangle

class NoteStrokeLineMarkerRenderer(private val startLine: Int, private val endLine: Int) : LineMarkerRenderer {
    override fun paint(editor: Editor, graphics: Graphics, rectangle: Rectangle) {
        // Start drawing from line 3 to line 5
        val lineHeight = editor.lineHeight

        // Calculate the Y position for the start and end lines
        val startY = rectangle.y + (startLine - 3) * lineHeight  // Adjusted for the starting line
        val endY = startY + ((endLine - startLine + 1) * lineHeight)

        // Drawing a vertical stroke in the gutter
        graphics.color = JBColor.BLUE  // Set the color of the stroke
        graphics.fillRect(rectangle.x, startY, rectangle.width, endY - startY)
    }
}
