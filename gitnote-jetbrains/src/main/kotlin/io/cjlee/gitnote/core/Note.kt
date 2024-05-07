package io.cjlee.gitnote.core

import com.fasterxml.jackson.annotation.JsonAlias
import com.fasterxml.jackson.annotation.JsonFormat
import com.fasterxml.jackson.core.JsonGenerator
import com.fasterxml.jackson.databind.JsonSerializer
import com.fasterxml.jackson.databind.SerializerProvider
import com.fasterxml.jackson.databind.annotation.JsonSerialize
import java.time.LocalDateTime

data class Note(
    val id: String,
    val reference: String,
    val messages: List<Message>
)

data class Message(
    val id: String,
    val line: Int,
    @JsonSerialize(using = MessageSerializer::class)
    val message: String,
    val snippet: String,
    @JsonFormat(shape = JsonFormat.Shape.STRING, pattern = "yyyy-MM-dd'T'HH:mm:ss'Z'")
    @JsonAlias("created_at")
    val createdAt: LocalDateTime,
)

class MessageSerializer : JsonSerializer<String>(){
    override fun serialize(str: String, gen: JsonGenerator, ser: SerializerProvider) {
        gen.writeString(str.replace(""""""", """\"""".trimIndent()))
    }
}
