package io.cjlee.gitnote.core

import org.apache.commons.io.FileUtils
import org.apache.commons.lang3.SystemUtils
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

    companion object ProcessLoader {
        private const val RESOURCE_LOCATION = "core/"
        private val BINARY_LOCATION = System.getProperty("java.io.tmpdir") + "git-note"
        val COMMAND: Array<String>

        private val isDevelopment = System.getProperty("gitnote.developmentPhase", "false").toBoolean()

        init {
            val classLoader = this::class.java.classLoader
            val file = File(BINARY_LOCATION)

            val os = determineOs()
            FileUtils.copyURLToFile(classLoader.getResource(RESOURCE_LOCATION + os.binary), file)

            COMMAND = if (!file.setExecutable(true) || isDevelopment) {
                arrayOf("git", "note")
            } else {
                arrayOf(BINARY_LOCATION)
            }

            file.deleteOnExit()
        }

        private fun determineOs(): OS {
            return when {
                SystemUtils.IS_OS_WINDOWS -> OS.WINDOWS
                SystemUtils.IS_OS_MAC -> OS.MAC
                SystemUtils.IS_OS_LINUX -> OS.LINUX
                else -> OS.UNKNOWN
            }
        }

        enum class OS(val os: String, val binary: String) {
            WINDOWS("windows", "x86_64_pc-windows-gnu/git-note.exe"),
            MAC("mac", "aarch64-apple-darwin/git-note"),
            LINUX("linux", "x86_64_unknown-linux-gnu/git-note"),
            UNKNOWN("unknown", "")
        }
    }
}
