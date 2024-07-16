import com.github.gradle.node.npm.task.NpxTask

plugins {
    id("java")
    id("org.jetbrains.kotlin.jvm") version "1.9.0"
    id("org.jetbrains.intellij") version "1.15.0"
    id("com.github.node-gradle.node") version "7.0.2"
}

group = "io.cjlee"
version = "0.0.2-P1"

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

val development = false

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
        systemProperty("gitnote.developmentPhase", development)
    }

    buildPlugin {
        if (!development) {
            dependsOn("buildCore")
            dependsOn("buildGui")
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

    register<NpxTask>("buildGui") {
        dependsOn("npmInstall") // Ensure npm is installed
        workingDir = file("../gitnote-gui") // Set the working directory to your React project
        command.set("npm")
        args.set(listOf("run", "build")) // Command to build the React project
    }

    register<Copy>("copyGui") {
        dependsOn(named("buildGui"))
        delete("src/main/resources/webview/")
        from("../gitnote-gui/build")
        into("src/main/resources/webview/.")
    }

    processResources {
        if (!development) {
            dependsOn(named("copyCore"))
            dependsOn(named("copyGui"))
        }
    }
}
