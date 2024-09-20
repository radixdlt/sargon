package com.radixdlt.cargo.desktop

import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.api.plugins.BasePlugin
import org.gradle.kotlin.dsl.create
import java.io.ByteArrayOutputStream

abstract class CargoDesktopConfiguration {
    var buildType: BuildType = BuildType.RELEASE
}

class CargoDesktopPlugin : Plugin<Project> {
    override fun apply(target: Project) {
        val extension = target.extensions.create<CargoDesktopConfiguration>("cargoDesktop")
        target.afterEvaluate {
            val buildType = extension.buildType

            val cargoTask = tasks.register("buildCargo${buildType.capitalised}") {
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
            }

            val cleanTask = tasks.register("cargoClean") {
                group = BasePlugin.BUILD_GROUP

                var currentTargetTriple: DesktopTargetTriple? = null
                doFirst {
                    currentTargetTriple = project.currentTargetTriple()
                }

                doLast {
                    val target = currentTargetTriple ?: return@doLast

                    exec {
                        workingDir = rootDir.parentFile
                        println("Cleaning for ${target.rustTargetTripleName}")
                        commandLine("cargo", "clean", "--target", target.rustTargetTripleName)
                    }
                }
            }

            tasks.getByName("compileJava") {
                dependsOn(cargoTask)
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