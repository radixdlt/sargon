import org.gradle.configurationcache.extensions.capitalized

plugins {
    id("java-library")
    id("maven-publish")
}

java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}

dependencies {
    implementation(libs.jna)
}

publishing {
    publications {
        register<MavenPublication>("release") {
            groupId = "com.radixdlt.sargon"
            artifactId = "sargon-desktop-bins"
            version = System.getenv("SARGON_VERSION")

            from(components["java"])
        }
    }

    repositories {
        maven {
            name = "GitHubPackages"
            url = uri("https://maven.pkg.github.com/radixdlt/sargon")
            credentials {
                username = System.getenv("GITHUB_ACTOR")
                password = System.getenv("GITHUB_TOKEN")
            }
        }
    }
}

sealed interface TargetTriple {

    val jnaName: String
    val binaryName: String
    val rustTargetTripleName: String
    object DarwinAArch64: TargetTriple {
        override val jnaName: String = "darwin-aarch64"
        override val binaryName: String = "libsargon.dylib"
        override val rustTargetTripleName: String = "aarch64-apple-darwin"
    }

    object DarwinX8664: TargetTriple {
        override val jnaName: String = "darwin-x86-64"
        override val binaryName: String = "libsargon.dylib"
        override val rustTargetTripleName: String = "x86_64-apple-darwin"
    }
    object LinuxArmel: TargetTriple {
        override val jnaName: String = "linux-armel"
        override val binaryName: String = "libsargon.so"
        override val rustTargetTripleName: String = "aarch64-unknown-linux-gnu"
    }
    object LinuxX8664: TargetTriple {
        override val jnaName: String = "linux-x86-64"
        override val binaryName: String = "libsargon.so"
        override val rustTargetTripleName: String = "x86_64-unknown-linux-gnu"
    }
    object LinuxWin32X8664: TargetTriple {
        override val jnaName: String = "win32-x86-64"
        override val binaryName: String = "sargon.dll"
        override val rustTargetTripleName: String = "x86_64-pc-windows-gnu"
    }

    companion object {
        val all = listOf(DarwinAArch64, DarwinX8664, LinuxArmel, LinuxX8664, LinuxWin32X8664)

        // Currently our CI supports building on Mac OS and we don't care about Mac Intel for now.
        // So we skip the rest.
        val supported = listOf(DarwinAArch64)
    }
}

listOf("debug", "release").forEach {
    val buildTypeUpper = it.capitalized()
    val buildType = it

    tasks.register("buildCargo$buildTypeUpper") {
        group = BasePlugin.BUILD_GROUP

        doFirst {
            TargetTriple.supported.forEach { triple ->
                exec {
                    commandLine("mkdir", "-p", "src/main/resources/${triple.jnaName}")
                }
            }
        }

        doLast {
            TargetTriple.all.forEach { triple ->
                println("Building for ${triple.rustTargetTripleName}")
                exec {
                    workingDir = projectDir.parentFile.parentFile
                    commandLine(
                        "cargo",
                        "build",
                        if (buildType == "release") "--release" else "",
                        "--target",
                        triple.rustTargetTripleName
                    )
                }

                exec {
                    workingDir = projectDir.parentFile.parentFile
                    commandLine(
                        "cp",
                        "target/${triple.rustTargetTripleName}/${buildType}/${triple.binaryName}",
                        "${rootDir}/${project.name}/src/main/resources/${triple.jnaName}/${triple.binaryName}"
                    )
                }
            }
        }
    }
}

tasks.getByName("compileJava") {
    dependsOn("buildCargoRelease")
}

tasks.register("cargoClean") {
    group = BasePlugin.BUILD_GROUP

    doLast {
        TargetTriple.all.forEach { triple ->
            exec {
                workingDir = rootDir.parentFile
                println("Cleaning for ${triple.rustTargetTripleName}")
                commandLine("cargo", "clean", "--target", triple.rustTargetTripleName)
            }
        }
    }
}

tasks.getByName("clean") {
    dependsOn("cargoClean")
}

