package io.cjlee.gitnote.core

import com.fasterxml.jackson.databind.DeserializationFeature
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue

class CoreHandler(private val connector: CoreConnector) {
    private val mapper = jacksonObjectMapper()
        .registerModule(JavaTimeModule())
        .configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false)

    // TODO : maybe cache would not be required, thanks to MergingUpdateQueue
    private val cache = NoteCache()

    fun read(filePath: String, force: Boolean = false): Note? {
        if (force) {
            return cache.put(filePath, read0(filePath))
        }
        return cache.get(filePath) ?: cache.put(filePath, read0(filePath))
    }

    fun readMessages(filePath: String, line: Int, force: Boolean = false): List<Message> {
        val note = read(filePath, force) ?: return emptyList()
        return note.messages.filter { it.line == line }
    }

    private fun read0(filePath: String): Note? {
        val response = connector.read(filePath)
        if (response.isSuccess) {
            return runCatching { mapper.readValue<Note>(response.text) }.getOrNull()
        }
        return null
    }

    // always do read on modification.
    fun add(filePath: String, line: Int, message: String): CoreConnector.Response {
        return connector.add(filePath, line + 1, message)
            .onSuccess { cache.put(filePath, read0(filePath)) }
    }

    // always do read on modification.
    fun update(filePath: String, line: Int, message: String): CoreConnector.Response {
        return connector.update(filePath, line + 1, message)
            .onSuccess { cache.put(filePath, read0(filePath)) }
    }

    // always do read on modification.
    fun delete(filePath: String, line: Int): CoreConnector.Response {
        return connector.delete(filePath, line + 1)
            .onSuccess { cache.put(filePath, read0(filePath)) }
    }

    private fun CoreConnector.Response.onSuccess(action: () -> Unit): CoreConnector.Response {
        if (isSuccess) {
            action()
        }
        return this
    }

    class NoteCache {
        private val notes = mutableMapOf<String, Note>()

        fun get(filePath: String): Note? {
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
