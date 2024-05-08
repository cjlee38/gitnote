package io.cjlee.gitnote.core

import com.fasterxml.jackson.databind.DeserializationFeature
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue

// TODO : cache
class CoreHandler(private val connector: CoreConnector) {
    private val mapper = jacksonObjectMapper()
        .registerModule(JavaTimeModule())
        .configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false)

    private val cache = NoteCache()

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

    fun delete(filePath: String, line: Int) {
        connector.delete(filePath, line)
    }

    fun readMessages(filePath: String, line: Int): List<Message> {
        val note = read(filePath) ?: return emptyList()
        return note.messages.filter { it.line == line }
    }

    private fun Response.onSuccess(action: Function1<*, *>) {
        TODO("Not yet implemented")
    }

    class NoteCache {
        private val notes = mutableMapOf<String, Note>()

        fun get(filePath: String): Note? {
            return notes[filePath]
        }

        fun put(filePath: String, note: Note) {
            notes[filePath] = note
        }
    }
}
