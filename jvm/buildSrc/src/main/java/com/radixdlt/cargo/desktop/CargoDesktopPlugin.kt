package com.radixdlt.cargo.desktop

import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.api.plugins.BasePlugin
import java.io.ByteArrayOutputStream

abstract class CargoDesktopConfiguration {
    var buildType: BuildType? = null
}

class CargoDesktopPlugin : Plugin<Project> {
    override fun apply(target: Project) {
        target.afterEvaluate {
            BuildType.values().forEach { buildType ->
                tasks.register("buildCargoDesktop${buildType.capitalised}") {
                    group = BasePlugin.BUILD_GROUP

                    var targetTriple: DesktopTargetTriple? = null
                    doFirst {
                        val current = project.currentTargetTriple().also {
                            targetTriple = it
                        }

                        exec {
                            commandLine("mkdir", "-p", "${layout.buildDirectory.get()}/generated/src/resources/${current.jnaName}")
                        }
                    }

                    doLast {
                        val current = targetTriple ?: return@doLast

                        println("Building for ${current.rustTargetTripleName}")
                        exec {
                            workingDir = projectDir.parentFile.parentFile
                            val commands = listOf(
                                "cargo",
                                "build",
                                "--locked",
                                "-p",
                                "sargon-uniffi",
                                if (buildType.isRelease()) "--release" else null,
                                "--target",
                                current.rustTargetTripleName
                            ).mapNotNull { it }

                            commandLine(commands)
                        }

                        exec {
                            workingDir = projectDir.parentFile.parentFile
                            commandLine(
                                "cp",
                                "target/${current.rustTargetTripleName}/${buildType.lowercase}/${current.binaryName}",
                                "${layout.buildDirectory.get()}/generated/src/resources/${current.jnaName}/${current.binaryName}"
                            )
                        }
                    }
                }
            }
        }
    }
}

fun Project.currentTargetTriple(): DesktopTargetTriple {
    val rustcVersion = providers.exec {
        commandLine("rustc", "--version", "--verbose")
    }.standardOutput.asText.get()

    val regex = "host: (.+)".toRegex()
    val host = regex.find(rustcVersion)
        ?.destructured
        ?.component1() ?: throw RuntimeException("No host found in $rustcVersion")

    return DesktopTargetTriple.from(host)
        ?: error("No compatible DesktopTargetTriple found called '$host'")
}