package io.cjlee.gitnote.core

import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader

class ProcessCoreConnector(
    private val projectPath: String
) : CoreConnector {
    private val runtime = Runtime.getRuntime()

    override fun add(filePath: String, startLine: Int, endLine: Int, message: String): Response {
        return executeCommand("git note add --file $filePath --line $startLine:$endLine --message \"$message\"")
    }

    override fun read(filePath: String): Response {
        return executeCommand("git note read --file $filePath --formatted")
    }

    override fun update(filePath: String, startLine: Int, endLine: Int, message: String): Response {
        return executeCommand("git note edit --file $filePath --line $startLine:$endLine --message \"$message\"")
    }

    override fun delete(filePath: String, startLine: Int, endLine: Int): Response {
        return executeCommand("git note delete --file $filePath --line $startLine:$endLine")
    }

    private fun executeCommand(command: String): Response {
        try {
            val process = runtime.exec(command, null, File(projectPath))
            process.waitFor()
            val exitValue = process.exitValue()
            val stream = if (exitValue == 0) process.inputStream else process.errorStream
            val reader = BufferedReader(InputStreamReader(stream))
            return Response(exitValue, reader.use { it.readText() })
        } catch (e: Exception) {
            e.printStackTrace()
            return Response(999, "")
        }
    }
}
