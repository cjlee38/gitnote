package io.cjlee.gitnote.core

import com.fasterxml.jackson.databind.DeserializationFeature
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue


class CoreHandler(private val connector: CoreConnector) {
    private val mapper = jacksonObjectMapper()
        .registerModule(JavaTimeModule())
        .configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false)

    fun add(filePath: String, line: Int, message: String) {
        connector.add(filePath, line, message)
    }

    fun read(filePath: String): Note? {
        val response = connector.read(filePath)
        if (response.exitCode == 0) {
            return runCatching { mapper.readValue<Note>(response.text) }.getOrNull()
        }
        return null
    }

    fun update(filePath: String, line: Int, message: String) {
        connector.update(filePath, line, message)
    }

    fun delete(filePath: String, startLine: Int, endLine: Int) {
        connector.delete(filePath, startLine)
    }
}
