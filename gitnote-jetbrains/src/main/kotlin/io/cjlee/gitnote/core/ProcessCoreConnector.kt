package io.cjlee.gitnote.core

import org.apache.commons.io.FileUtils
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
            process.waitFor()
            val exitValue = process.exitValue()
            val stream = if (exitValue == 0) process.inputStream else process.errorStream
            val reader = BufferedReader(InputStreamReader(stream))
            CoreConnector.Response(exitValue, reader.use { it.readText() })
        } catch (e: Exception) {
            e.printStackTrace()
            CoreConnector.Response(999, "")
        }
    }

    companion object ProcessLoader {
        private const val RESOURCE_LOCATION = "core/git-note"
        private val BINARY_LOCATION = System.getProperty("java.io.tmpdir") + "git-note"
        val COMMAND: Array<String>

        init {
            val classLoader = this::class.java.classLoader
            val file = File(BINARY_LOCATION)
            if (!file.exists()) {
                FileUtils.copyURLToFile(classLoader.getResource(RESOURCE_LOCATION), file)
            }

            COMMAND = if (!file.setExecutable(true)) { arrayOf("git", "note") }
            else { arrayOf(BINARY_LOCATION) }

            file.deleteOnExit()
        }
    }
}
