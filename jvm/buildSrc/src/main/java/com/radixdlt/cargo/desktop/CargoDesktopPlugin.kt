package com.radixdlt.cargo.desktop

import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.api.plugins.BasePlugin
import org.gradle.kotlin.dsl.create

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

                doFirst {
                    DesktopTargetTriple.supported.forEach { triple ->
                        exec {
                            commandLine("mkdir", "-p", "src/main/resources/${triple.jnaName}")
                        }
                    }
                }

                doLast {
                    DesktopTargetTriple.supported.forEach { triple ->
                        println("Building for ${triple.rustTargetTripleName}")
                        exec {
                            workingDir = projectDir.parentFile.parentFile
                            val commands = listOf(
                                "cargo",
                                "build",
                                if (buildType.isRelease()) "--release" else null,
                                "--target",
                                triple.rustTargetTripleName
                            ).mapNotNull { it }

                            commandLine(commands)
                        }

                        exec {
                            workingDir = projectDir.parentFile.parentFile
                            commandLine(
                                "cp",
                                "target/${triple.rustTargetTripleName}/${buildType.lowercase}/${triple.binaryName}",
                                "${projectDir}/src/main/resources/${triple.jnaName}/${triple.binaryName}"
                            )
                        }
                    }
                }
            }

            val cleanTask = tasks.register("cargoClean") {
                group = BasePlugin.BUILD_GROUP

                doLast {
                    DesktopTargetTriple.supported.forEach { triple ->
                        exec {
                            workingDir = rootDir.parentFile
                            println("Cleaning for ${triple.rustTargetTripleName}")
                            commandLine("cargo", "clean", "--target", triple.rustTargetTripleName)
                        }
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
}