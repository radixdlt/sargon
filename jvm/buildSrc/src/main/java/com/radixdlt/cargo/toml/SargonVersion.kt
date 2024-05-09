package com.radixdlt.cargo.toml

import org.gradle.api.Project
import org.tomlj.Toml
import java.io.ByteArrayOutputStream
import java.io.File

private fun Project.parseTomlVersion(): String {
    val tomlFile = File(projectDir.parentFile.parentFile, "Cargo.toml").toPath()

    val toml = Toml.parse(tomlFile)
    return toml.getString("package.version").orEmpty()
}

private fun Project.parseGitHash(): String {
    val out = ByteArrayOutputStream()
    exec {
        commandLine("git", "rev-parse", "--short", "@")
        standardOutput = out
    }.assertNormalExitValue()
    return String(out.toByteArray(), Charsets.UTF_8).trim()
}

fun Project.sargonVersion(): String = "${parseTomlVersion()}-${parseGitHash()}"