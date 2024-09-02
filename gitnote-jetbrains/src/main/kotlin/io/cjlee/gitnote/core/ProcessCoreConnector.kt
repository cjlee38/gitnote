package io.cjlee.gitnote.core

import org.apache.commons.io.FileUtils
import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader
import java.nio.file.Files

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

    companion object ProcessLoader {
        private const val RESOURCE_LOCATION = "core/"
        private val systemCommand = arrayOf("git", "note")
        val COMMAND: Array<String>

        private val isDevelopment = System.getProperty("gitnote.developmentPhase", "false").toBoolean()

        init {
            val platform = Platform.determine()
            COMMAND = command(platform)
        }

        private fun command(platform: Platform?): Array<String> {
            if (isDevelopment || platform == null) {
                return systemCommand
            }
            val file = loadFile(platform)
            if (!file.setExecutable(true)) {
                return systemCommand
            }
            return arrayOf(file.path)
        }

        private fun loadFile(platform: Platform): File {
            val classLoader = this::class.java.classLoader
            return Files.createTempFile("git-note", ".tmp")
                .toFile()
                .apply {
                    this.deleteOnExit()
                    val resource = classLoader.getResource(RESOURCE_LOCATION + platform.binary)
                    FileUtils.copyURLToFile(resource, this)
                }
        }
    }
}
