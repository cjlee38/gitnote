package io.cjlee.gitnote.core

import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader

class ProcessCoreConnector(
    private val projectPath: String
) : CoreConnector {
    private val command
        get() = COMMAND
    private val runtime = Runtime.getRuntime()

    override fun add(filePath: String, line: Int, message: String): CoreConnector.Response {
        return executeCommand("add", "--file", filePath, "--line", "$line", "--message", message)
    }

    override fun read(filePath: String): CoreConnector.Response {
        return executeCommand("read", "--file", filePath, "--formatted")
    }

    override fun update(filePath: String, line: Int, message: String): CoreConnector.Response {
        return executeCommand("edit", "--file", filePath, "--line", "$line", "--message", message)
    }

    override fun delete(filePath: String, line: Int): CoreConnector.Response {
        return executeCommand("delete", "--file", filePath, "--line", "$line")
    }

    private fun executeCommand(vararg command: String): CoreConnector.Response {
        return try {
            val process = runtime.exec(this.command + command, null, File(projectPath))
            System.err.println("execute command : ${(this.command + command).contentToString()}")
            process.waitFor()
            val exitValue = process.exitValue()
            val stream = if (exitValue == 0) process.inputStream else process.errorStream
            val reader = BufferedReader(InputStreamReader(stream))
            CoreConnector.Response(exitValue, reader.use { it.readText() })
                .also { System.err.println("command execution : $it") }
        } catch (e: Exception) {
            e.printStackTrace()
            CoreConnector.Response(999, "")
        }
    }

    companion object {
        private val phase = System.getProperty("gitnote.phase", "1").toInt()
        private val systemCommand = arrayOf("git", "note")
        val COMMAND: Array<String>

        init {
            COMMAND = determineCommand()
        }

        private fun determineCommand(): Array<String> {
            val platform = ConnectorLoader.platform
            if (phase == 0 || platform == null) {
                return systemCommand
            }
            val filename = when (platform) {
                Platform.WINDOWS -> "git-note.exe"
                else -> "git-note"
            }

            val file = ConnectorLoader.loadFile(platform.arch + "/" + filename)
            if (!file.setExecutable(true)) {
                return systemCommand
            }
            return arrayOf(file.path)
        }
    }
}
