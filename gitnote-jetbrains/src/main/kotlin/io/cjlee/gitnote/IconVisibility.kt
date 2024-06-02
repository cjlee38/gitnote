package io.cjlee.gitnote

import com.intellij.openapi.editor.Editor
import com.intellij.openapi.editor.LogicalPosition
import com.intellij.openapi.editor.event.EditorMouseEvent
import com.intellij.openapi.editor.event.EditorMouseListener
import com.intellij.openapi.editor.event.EditorMouseMotionListener
import com.intellij.openapi.editor.ex.RangeHighlighterEx
import javax.swing.JComponent

class IconVisibility(
    private val highlighters: Set<RangeHighlighterEx>
) : EditorMouseListener, EditorMouseMotionListener {

    override fun mouseMoved(e: EditorMouseEvent) = doUpdate(e.editor, e.logicalPosition.line)
    override fun mouseExited(e: EditorMouseEvent) = doUpdate(e.editor, -1)

    private fun doUpdate(editor: Editor, line: Int) {
        highlighters.mapNotNull { it.gutterIconRenderer as? GitNoteGutterIconRenderer }.forEach {
            val visible = it.line == line
            val needUpdate = it.visible != visible && !it.hasMessage
            if (needUpdate) {
                it.visible = visible
                val gutter = editor.gutter as JComponent
                val y = editor.logicalPositionToXY(LogicalPosition(it.line, 0)).y
                gutter.repaint(0, y, gutter.width, y + editor.lineHeight)
            }
        }
    }
}
