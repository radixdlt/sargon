package com.radixdlt.cargo.toml

import org.gradle.api.Project
import org.tomlj.Toml
import java.io.ByteArrayOutputStream
import java.io.File

private fun Project.parseTomlVersion(): String {
    val sargonCrate = File(projectDir.parentFile.parentFile, "crates/sargon")
    val tomlFile = File(sargonCrate, "Cargo.toml").toPath()

    val toml = Toml.parse(tomlFile)
    return toml.getString("package.version").orEmpty()
}

private fun Project.parseGitHash(): String = providers.exec {
    commandLine("git", "rev-parse", "--short", "@")
}.standardOutput.asText.get().replace("\n", "")

fun Project.sargonVersion(isDebug: Boolean): String {
    val customBuildName = System.getenv("CUSTOM_BUILD_NAME")?.takeIf {
        it.isNotBlank()
    }?.replace("\\s+".toRegex(), "-")?.let {
        "-${it}"
    }.orEmpty()

    val snapshot = if (isDebug) "-SNAPSHOT" else ""

    return "${parseTomlVersion()}${customBuildName}-${parseGitHash()}$snapshot"
}