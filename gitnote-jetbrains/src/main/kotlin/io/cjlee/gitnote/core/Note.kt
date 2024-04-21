package io.cjlee.gitnote.core

import com.fasterxml.jackson.annotation.JsonAlias
import com.fasterxml.jackson.annotation.JsonFormat
import java.time.LocalDateTime

data class Note(
    val id: String,
    val reference: String,
    val messages: List<Message>
)

data class Message(
    val id: String,
    @JsonAlias("start")
    val startLine: Int,
    @JsonAlias("end")
    val endLine: Int,
    val message: String,
    val snippet: List<String>,
    @JsonFormat(shape = JsonFormat.Shape.STRING, pattern = "yyyy-MM-dd'T'HH:mm:ss'Z'")
    @JsonAlias("created_at")
    val createdAt: LocalDateTime,
)
