package io.cjlee.gitnote.jcef

import com.intellij.openapi.components.Service
import com.intellij.openapi.project.Project
import io.cjlee.gitnote.jcef.protocol.ProtocolHandler

@Service(Service.Level.PROJECT)
class JcefViewerWindowService(private val project: Project) {
    fun newWindow(protocolHandlers: Map<String, ProtocolHandler>): GitNoteViewerWindow {
        return GitNoteViewerWindow(project, protocolHandlers)
    }
}
