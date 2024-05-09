import com.radixdlt.cargo.desktop.BuildType
import java.io.ByteArrayOutputStream

plugins {
    id("java-library")
    id("maven-publish")
    id("com.radixdlt.cargo.desktop")
}

java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}

dependencies {
    implementation("net.java.dev.jna:jna:5.13.0")

    // For Async support
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core-jvm:1.8.0")
    // For Network support
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
    implementation("com.squareup.okio:okio:3.7.0") 
}

publishing {
    publications {

        fun command(command: String): String {
            val out = ByteArrayOutputStream()
            exec {
                commandLine(command.split(" "))
                standardOutput = out
            }.assertNormalExitValue()
            return String(out.toByteArray(), Charsets.UTF_8).trim()
        }

        register<MavenPublication>("release") {
            groupId = "com.radixdlt.sargon"
            artifactId = "sargon-desktop-bins"

            val toml = File(projectDir.parentFile.parentFile, "Cargo.toml").readText()
            val matchResult: MatchResult? = "version\\s*=\\s*\"(.+)\"".toRegex().find(toml)
            version = matchResult?.let {
                val (version) = matchResult.destructured

                version
            } ?: run {
                command("git tag --sort=committerdate").split("\n").last()
            }.let { version ->
                val commitHash = command("git rev-parse --short @")

                if (commitHash.isNotBlank()) {
                    "$version-$commitHash"
                } else {
                    version
                }
            }

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

cargoDesktop {
    buildType = BuildType.RELEASE
}

