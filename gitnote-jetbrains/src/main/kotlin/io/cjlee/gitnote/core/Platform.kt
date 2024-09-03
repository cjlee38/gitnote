package io.cjlee.gitnote.core

import org.apache.commons.lang3.SystemUtils

enum class Platform(val binary: String) {
    WINDOWS("x86_64_pc-windows-gnu/git-note.exe"),
    INTEL_MAC("x86_64-apple-darwin/git-note"),
    SILICON_MAC("aarch64-apple-darwin/git-note"),
    LINUX("x86_64_unknown-linux-gnu/git-note");

    companion object {
        fun determine(): Platform? {
            return when {
                SystemUtils.IS_OS_WINDOWS -> WINDOWS
                SystemUtils.IS_OS_MAC -> {
                    val arch = SystemUtils.OS_ARCH
                    if (arch.contains("aarch64")) SILICON_MAC else INTEL_MAC
                }
                SystemUtils.IS_OS_LINUX -> LINUX
                else -> null
            }.also { println("determined platform : $it") }
        }
    }
}
