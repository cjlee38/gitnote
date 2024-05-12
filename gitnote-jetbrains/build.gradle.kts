import com.github.gradle.node.npm.task.NpxTask

plugins {
    id("java")
    id("org.jetbrains.kotlin.jvm") version "1.9.0"
    id("org.jetbrains.intellij") version "1.15.0"
    id("com.github.node-gradle.node") version "7.0.2"
}

group = "io.cjlee"
version = "1.0-SNAPSHOT"

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
        untilBuild.set("232.*")
    }

    runIde {
        autoReloadPlugins = true
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
        commandLine("cargo", "build", "--release")
    }

    register<Copy>("copyCore") {
        dependsOn(named("buildCore"))
        delete("src/main/resources/core/")
        from("../gitnote-core/target/release/git-note")
        into("src/main/resources/core/.")
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

// TODO : download node and npm if not present
//node {
//    version.set("22.0.0")
//    npmVersion.set("10.5.1")
//    download.set(true)
//    workDir.set(file("${project.buildDir}/nodejs"))
//    npmWorkDir.set(file("${project.layout.buildDirectory}/npm"))
//}
