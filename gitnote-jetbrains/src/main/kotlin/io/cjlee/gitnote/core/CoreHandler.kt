package io.cjlee.gitnote.core

import com.fasterxml.jackson.databind.DeserializationFeature
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue


class CoreHandler(private val connector: CoreConnector) {
    private val mapper = jacksonObjectMapper()
        .registerModule(JavaTimeModule())
        .configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false)

    fun add(filePath: String, startLine: Int, endLine: Int, message: String) {
        connector.add(filePath, startLine, endLine, message)
    }

    fun read(filePath: String): Note? {
        val response = connector.read(filePath)
        if (response.exitCode == 0) {
            return runCatching { mapper.readValue<Note>(response.text) }.getOrNull()
        }
//        println("=== ERROR read failed. response was $response")
        return null
    }

    fun update(filePath: String, startLine: Int, endLine: Int, message: String) {
        println("=====update filePath = $filePath, startLine = $startLine, endLine = $endLine, message = $message")

//        connector.update(filePath, startLine, endLine, message)
    }

    fun delete(filePath: String, startLine: Int, endLine: Int) {
        connector.delete(filePath, startLine, endLine)
    }
}
