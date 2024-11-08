import com.github.gradle.node.npm.task.NpxTask

plugins {
    id("java")
    id("org.jetbrains.kotlin.jvm") version "1.9.0"
    id("org.jetbrains.intellij") version "1.15.0"
    id("com.github.node-gradle.node") version "7.0.2"
}

group = "io.cjlee"
version = "0.3.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.fasterxml.jackson.module:jackson-module-kotlin:2.17.0")
    implementation("com.fasterxml.jackson.core:jackson-core:2.17.0")
    implementation("com.fasterxml.jackson.datatype:jackson-datatype-jsr310:2.17.0")
}

// Configure Gradle IntelliJ Plugin
// Read more: https://plugins.jetbrains.com/docs/intellij/tools-gradle-intellij-plugin.html
intellij {
    version.set("2022.2.5")
    type.set("IC") // Target IDE Platform

    plugins.set(listOf(/* Plugin Dependencies */))
}


/**
 * Determine whether to use `localhost:3000` GUI for development. false by default means using the built-in GUI
 */
val useLocalGui = System.getProperty("gitnote.useLocalGui")?.toBoolean() ?: false

/**
 * Determine whether to build the core module locally.
 *
 * Usually set to false when you release the plugin using github-actions.
 */
val buildCore = System.getProperty("gitnote.buildCore")?.toBoolean() ?: true

tasks {
    // Set the JVM compatibility versions
    withType<JavaCompile> {
        sourceCompatibility = "17"
        targetCompatibility = "17"
    }
    withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
        kotlinOptions.jvmTarget = "17"
    }

    patchPluginXml {
        sinceBuild.set("222")
        untilBuild.set("241.*")
    }

    runIde {
        autoReloadPlugins = true
    }

    buildPlugin {
        if (!useLocalGui) {
            dependsOn("buildGui")
        }
        if (buildCore) {
            dependsOn("buildCore")
        }
    }

    processResources {
        if (!useLocalGui) {
            dependsOn(named("copyGui"))
        }
        if (buildCore) {
            dependsOn(named("copyCore"))
        }
    }

    signPlugin {
        certificateChain.set(System.getenv("CERTIFICATE_CHAIN"))
        privateKey.set(System.getenv("PRIVATE_KEY"))
        password.set(System.getenv("PRIVATE_KEY_PASSWORD"))
    }

    publishPlugin {
        token.set(System.getenv("PUBLISH_TOKEN"))
    }

    register<Exec>("buildCore") {
        workingDir = file("../gitnote-core")
        commandLine("sh", "build.sh")
    }

    register<Exec>("copyCore") {
        dependsOn(named("buildCore"))
        workingDir = file("../gitnote-core")
        commandLine("sh", "copy.sh")
    }

    npmInstall {
        workingDir = file("../gitnote-gui")
    }

    register<NpxTask>("buildGui") {
        dependsOn("npmInstall")
        workingDir = file("../gitnote-gui")
        command.set("npm")
        args.set(listOf("run", "build"))
    }

    register<Copy>("copyGui") {
        dependsOn(named("buildGui"))
        delete("src/main/resources/webview/")
        from("../gitnote-gui/build")
        into("src/main/resources/webview/.")
    }
}

node {
    version = "22.2.0"
    npmVersion = "10.7.0"
    download = !useLocalGui
    nodeProjectDir = file("../gitnote-gui")
}
