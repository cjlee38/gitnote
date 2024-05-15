package io.cjlee.gitnote.core

import com.fasterxml.jackson.databind.DeserializationFeature
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue

class CoreHandler(private val connector: CoreConnector) {
    private val mapper = jacksonObjectMapper()
        .registerModule(JavaTimeModule())
        .configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false)

    private val cache = NoteCache()

    fun read(filePath: String, force: Boolean = false): Note? {
        if (force) {
            println("forced")
            return cache.put(filePath, read0(filePath))
        }
        return cache.get(filePath) ?: cache.put(filePath, read0(filePath))
    }

    fun readMessages(filePath: String, line: Int, force: Boolean = false): List<Message> {
        val note = read(filePath, force) ?: return emptyList()
        return note.messages.filter { it.line == line }
    }

    private fun read0(filePath: String): Note? {
        println("real read $filePath")
        val response = connector.read(filePath)
        if (response.exitCode == 0) {
            return runCatching { mapper.readValue<Note>(response.text) }.getOrNull()
        }
        return null
    }

    // always do read on modification.
    fun add(filePath: String, line: Int, message: String) {
        connector.add(filePath, line, message)
            .onSuccess { cache.put(filePath, read0(filePath)) }
    }

    // always do read on modification.
    fun update(filePath: String, line: Int, message: String) {
        connector.update(filePath, line, message)
            .onSuccess { cache.put(filePath, read0(filePath)) }
    }

    // always do read on modification.
    fun delete(filePath: String, line: Int) {
        connector.delete(filePath, line)
            .onSuccess { cache.put(filePath, read0(filePath)) }
    }

    private fun Response.onSuccess(action: () -> Unit) {
        if (exitCode == 0) {
            println("successs")
            action()
        }
        println("no-successs")
    }

    class NoteCache {
        private val notes = mutableMapOf<String, Note>()

        fun get(filePath: String): Note? {
            println("cache read $filePath && ${notes[filePath] != null}")
            return notes[filePath]
        }

        fun put(filePath: String, note: Note?): Note? {
            if (note != null) {
                notes[filePath] = note
                return note
            }
            return null
        }
    }
}
