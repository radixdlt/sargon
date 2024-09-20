package com.radixdlt.cargo.desktop

import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.api.plugins.BasePlugin
import org.gradle.kotlin.dsl.create
import java.io.ByteArrayOutputStream

abstract class CargoDesktopConfiguration {
    var buildType: BuildType? = null
}

class CargoDesktopPlugin : Plugin<Project> {
    override fun apply(target: Project) {
        target.afterEvaluate {
            BuildType.values().forEach { buildType ->
                tasks.register("buildCargo${buildType.capitalised}") {
                    group = BasePlugin.BUILD_GROUP

                    var targetTriple: DesktopTargetTriple? = null
                    doFirst {
                        val current = project.currentTargetTriple().also {
                            targetTriple = it
                        }

                        exec {
                            commandLine("mkdir", "-p", "src/main/resources/${current.jnaName}")
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
                                "${projectDir}/src/main/resources/${current.jnaName}/${current.binaryName}"
                            )
                        }
                    }

                    onlyIf {
                        val skipCargo = (properties["skip-cargo"] as? String)
                            ?.toBooleanStrictOrNull() ?: false
                        !skipCargo
                    }
                }
            }

            val cleanTask = tasks.register("cargoClean") {
                group = BasePlugin.BUILD_GROUP

                var currentTargetTriple: DesktopTargetTriple? = null
                doFirst {
                    currentTargetTriple = project.currentTargetTriple()
                }

                doLast {
                    val targetTriple = currentTargetTriple ?: return@doLast

                    exec {
                        workingDir = rootDir.parentFile
                        println("Cleaning for ${targetTriple.rustTargetTripleName}")
                        commandLine("cargo", "clean", "--target", targetTriple.rustTargetTripleName)
                    }
                }
            }

            tasks.getByName("compileJava") {
                val extension = target.extensions.create<CargoDesktopConfiguration>("cargoDesktop")
                val buildType = extension.buildType
                    ?: BuildType.from(properties["buildType"] as String?) ?: BuildType.DEBUG

                dependsOn(tasks.getByName("buildCargo${buildType.capitalised}"))
            }

            tasks.getByName("clean") {
                dependsOn(cleanTask)
            }
        }
    }

    private fun Project.currentTargetTriple(): DesktopTargetTriple {
        val rustcVersion: String = ByteArrayOutputStream().use { outputStream ->
            project.exec {
                commandLine("rustc", "--version", "--verbose")
                standardOutput = outputStream
            }
            outputStream.toString()
        }

        val regex = "host: (.+)".toRegex()
        val host = regex.find(rustcVersion)
            ?.destructured
            ?.component1() ?: throw RuntimeException("No host found in $rustcVersion")

        return DesktopTargetTriple.current(host)
    }
}