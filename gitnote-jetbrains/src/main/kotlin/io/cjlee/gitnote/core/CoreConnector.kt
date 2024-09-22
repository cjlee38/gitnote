package io.cjlee.gitnote.core

import com.fasterxml.jackson.annotation.JsonAlias

interface CoreConnector {
    fun add(filePath: String, line: Int, message: String): Response
    fun read(filePath: String): Response
    fun update(filePath: String, line: Int, message: String): Response
    fun delete(filePath: String, line: Int): Response

    data class Response(
        @JsonAlias("exit_code")
        val exitCode: Int,
        val text: String, // todo : rename to payload
    ) {
        val isSuccess: Boolean
            get() = exitCode == 0
    }
}
