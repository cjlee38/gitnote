package io.cjlee.gitnote.core

interface CoreConnector {
    fun add(filePath: String, line: Int, message: String): Response
    fun read(filePath: String): Response
    fun update(filePath: String, line: Int, message: String): Response
    fun delete(filePath: String, line: Int): Response
}

data class Response(
    val exitCode: Int,
    val text: String,
)
