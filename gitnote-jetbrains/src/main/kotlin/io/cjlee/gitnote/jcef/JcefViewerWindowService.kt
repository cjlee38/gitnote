package io.cjlee.gitnote.jcef

import com.intellij.openapi.components.Service
import com.intellij.openapi.project.Project
import io.cjlee.gitnote.jcef.protocol.MessageProtocolHandler

@Service(Service.Level.PROJECT)
class JcefViewerWindowService(private val project: Project) {
    fun newWindow(protocolHandlers: Map<String, MessageProtocolHandler>): GitNoteViewerWindow {
        return GitNoteViewerWindow(project, protocolHandlers)
    }
}
