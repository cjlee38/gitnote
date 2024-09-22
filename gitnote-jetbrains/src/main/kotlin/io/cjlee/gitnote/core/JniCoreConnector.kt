package io.cjlee.gitnote.core

import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import java.io.File

class JniCoreConnector(
    private val projectPath: String
) : CoreConnector {
    private val mapper = jacksonObjectMapper().registerModule(JavaTimeModule())

    companion object {
        val libFile: File by lazy {
            val platform = ConnectorLoader.platform ?: throw IllegalStateException("platform is not supported")
            val libName = when (platform) {
                Platform.WINDOWS -> "libgitnote.dll"
                Platform.LINUX -> "libgitnote.so"
                Platform.INTEL_MAC -> "libgitnote.dylib"
                Platform.SILICON_MAC -> "libgitnote.dylib"
            }
            ConnectorLoader.loadFile(platform.arch + "/" + libName)
        }
    }

    init {
        System.load(libFile.path)
    }

    override fun add(filePath: String, line: Int, message: String): CoreConnector.Response {
        return add0(projectPath, filePath, line, message)
            .let { mapper.readValue<CoreConnector.Response>(it) }
    }

    private external fun add0(execPath: String, filePath: String, line: Int, message: String): String

    override fun read(filePath: String): CoreConnector.Response {
        return read0(projectPath, filePath)
            .let { mapper.readValue<CoreConnector.Response>(it) }
    }

    private external fun read0(execPath: String, filePath: String): String

    override fun update(filePath: String, line: Int, message: String): CoreConnector.Response {
        return update0(projectPath, filePath, line, message)
            .let { mapper.readValue<CoreConnector.Response>(it) }
    }

    private external fun update0(execPath: String, filePath: String, line: Int, message: String): String

    override fun delete(filePath: String, line: Int): CoreConnector.Response {
        return delete0(projectPath, filePath, line)
            .let { mapper.readValue<CoreConnector.Response>(it) }
    }

    private external fun delete0(execPath: String, filePath: String, line: Int): String
}
