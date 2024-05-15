package io.cjlee.gitnote.core

import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader

class ProcessCoreConnector(
    private val projectPath: String
) : CoreConnector {
    private val command = arrayOf("git", "note")
    private val runtime = Runtime.getRuntime()

    override fun add(filePath: String, line: Int, message: String): Response {
        return executeCommand("add", "--file", filePath, "--line", "$line", "--message", message, "--stage")
    }

    override fun read(filePath: String): Response {
        return executeCommand("read", "--file", filePath, "--formatted")
    }

    override fun update(filePath: String, line: Int, message: String): Response {
        return executeCommand("edit", "--file", filePath, "--line", "$line", "--message", message)
    }

    override fun delete(filePath: String, line: Int): Response {
        return executeCommand("delete", "--file", filePath, "--line", "$line")
    }

    private fun executeCommand(vararg command: String): Response {
        return try {
            val process = runtime.exec(this.command + command, null, File(projectPath))
            process.waitFor()
            val exitValue = process.exitValue()
            val stream = if (exitValue == 0) process.inputStream else process.errorStream
            val reader = BufferedReader(InputStreamReader(stream))
            Response(exitValue, reader.use { it.readText() })
        } catch (e: Exception) {
            e.printStackTrace()
            Response(999, "")
        }
    }
}
