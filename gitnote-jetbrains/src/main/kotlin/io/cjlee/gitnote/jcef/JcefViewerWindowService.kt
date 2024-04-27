package io.cjlee.gitnote.jcef

import com.intellij.openapi.components.Service
import com.intellij.openapi.project.Project

@Service(Service.Level.PROJECT)
class JcefViewerWindowService(project: Project) {
    val gitNoteViewerWindow = GitNoteViewerWindow(project)
}
