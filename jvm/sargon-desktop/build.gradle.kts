import com.radixdlt.cargo.desktop.DesktopTargetTriple
import com.radixdlt.cargo.toml.sargonVersion

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
        register<MavenPublication>("release") {
            groupId = "com.radixdlt.sargon"
            artifactId = "sargon-desktop"
            version = project.sargonVersion()

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

tasks.register("copyGithubArtifacts") {
    doFirst {
        DesktopTargetTriple.ciSupported.forEach { triple ->
            exec {
                commandLine("mkdir", "-p", "src/main/resources/${triple.jnaName}")
            }
        }
    }

    doLast {
        val rustProjectDir = projectDir.parentFile.parentFile
        val artifactsDir = File(rustProjectDir, "artifacts")

        val fileTargets = artifactsDir.listFiles()?.map { file ->
            file to DesktopTargetTriple.current(file.name)
        }.orEmpty()

        fileTargets.forEach { target ->
            exec {
                workingDir = rustProjectDir
                commandLine(
                    "cp",
                    "${target.first.canonicalPath}/${target.second.binaryName}",
                    "${projectDir}/src/main/resources/${target.second.jnaName}/${target.second.binaryName}"
                )
            }
        }
    }
}