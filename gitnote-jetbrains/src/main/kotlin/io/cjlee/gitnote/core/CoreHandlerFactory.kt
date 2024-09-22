package io.cjlee.gitnote.core

object CoreHandlerFactory {
    private val coreHandlers = mutableMapOf<String, CoreHandler>()

    fun get(projectPath: String): CoreHandler {
        return coreHandlers[projectPath] ?: createCoreHandler(projectPath)
    }

    private fun createCoreHandler(projectPath: String): CoreHandler {
        return CoreHandler(JniCoreConnector(projectPath)).also { coreHandlers[projectPath] = it }
//        return CoreHandler(ProcessCoreConnector(projectPath)).also { coreHandlers[projectPath] = it }
    }
}
