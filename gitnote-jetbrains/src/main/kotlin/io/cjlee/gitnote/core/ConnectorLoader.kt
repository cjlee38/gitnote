package io.cjlee.gitnote.core

import org.apache.commons.io.FileUtils
import java.io.File
import java.nio.file.Files

object ConnectorLoader {
    private const val RESOURCE_LOCATION = "core/"
    val platform = Platform.determine()

    fun loadFile(filepath: String): File {
        val classLoader = this::class.java.classLoader
        return Files.createTempFile("git-note", ".tmp")
            .toFile()
            .apply {
                this.deleteOnExit()
                val resource = classLoader.getResource(RESOURCE_LOCATION + filepath)
                FileUtils.copyURLToFile(resource, this)
            }
    }
}
