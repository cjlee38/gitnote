package io.cjlee.gitnote.core

interface CoreConnector {
    fun add(filePath: String, startLine: Int, endLine: Int, message: String): Response
    fun read(filePath: String): Response
    fun update(filePath: String, startLine: Int, endLine: Int, message: String): Response
    fun delete(filePath: String, startLine: Int, endLine: Int): Response
}

data class Response(
    val exitCode: Int,
    val text: String,
)
