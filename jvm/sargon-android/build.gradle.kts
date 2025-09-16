import com.radixdlt.cargo.desktop.DesktopTargetTriple
import com.radixdlt.cargo.desktop.currentTargetTriple
import com.radixdlt.cargo.toml.sargonVersion
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import java.nio.file.Files

plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.serialization)
    alias(libs.plugins.android.cargo.ndk)
    alias(libs.plugins.kotlin.kover)
    id("com.radixdlt.cargo.desktop")
    id("maven-publish")
}

android {
    namespace = "com.radixdlt.sargon"
    compileSdk = libs.versions.sdk.compile.get().toInt()

    defaultConfig {
        minSdk = libs.versions.sdk.min.get().toInt()

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }

    sourceSets {
        getByName("main") {
            java.srcDir("${layout.buildDirectory.get()}/generated/src/kotlin")
        }

        configureEach {
            if (name.startsWith("test")) {
                // Unit tests need to be linked with "desktop", native to the machine, libraries
                resources.srcDirs("${layout.buildDirectory.get()}/generated/src/resources")
            } else {
                // The rest of the tasks are android related and need to link
                // with the android jniLibs
                jniLibs.srcDirs("${layout.buildDirectory.get()}/generated/src/jniLibs")
            }
        }
    }

    packaging {
        // Need to merge junit 4 & 5 licences
        resources.merges.addAll(
            listOf(
                "META-INF/LICENSE.md",
                "META-INF/LICENSE-notice.md",
            )
        )
    }

    buildTypes.forEach {
        val buildTypeVariant = it.name.replaceFirstChar(Char::uppercase)
        tasks.register<Jar>("desktopJar${buildTypeVariant}") {
            from("${layout.buildDirectory.get()}/generated/src/resources")

            if (it.isDebuggable) {
                // For debug we only need to build for the current architecture. Used by maven publication in
                // debug mode.
                dependsOn("buildCargoDesktopDebug")
            } else {
                // This task is used when publishing `sargon-desktop-bins`.
                // Before generating a Jar we need all native libs to have been built for all desktop
                // architectures.
                // The building is handled by github. After that the `copyExternalArtifacts` needs to copy
                // all the built libraries into the resources directory.
                dependsOn("copyExternalArtifacts")
            }
        }
    }
}

cargoNdk {
    targets = arrayListOf("arm64", "arm")
    module = "../"
    librariesNames = arrayListOf("libsargon_uniffi.so")
    extraCargoBuildArguments = arrayListOf("--locked", "--all")
}

tasks.withType<Test> {
    useJUnitPlatform()
    // Need to specifically set the path for JNA
    // The binary file is not automatically included in the classpath
    val triple = project.currentTargetTriple()
    systemProperties["jna.library.path"] = "${layout.buildDirectory.get()}/generated/src/resources/${triple.jnaName}"
}

tasks.withType<KotlinCompile>().configureEach {
    if (name.contains("Test")) {
        kotlinOptions.freeCompilerArgs += "-Xopt-in=com.radixdlt.sargon.annotation.UsesSampleValues"
    }
}

koverReport {
    filters {
        excludes {
            packages("com.radixdlt.sargon.samples")
            // Drivers are tested in androidTest
            packages("com.radixdlt.sargon.os.driver")
            annotatedBy("com.radixdlt.sargon.annotation.KoverIgnore")
        }
        includes {
            packages("com.radixdlt.sargon.extensions")
            packages("com.radixdlt.sargon.antenna")
            packages("com.radixdlt.sargon.os")
        }
    }

    verify {
        rule {
            minBound(100)
        }
    }
}

dependencies {
    implementation("${libs.jna.get()}@aar")

    // For lifecycle callbacks
    implementation(libs.androidx.appcompat)
    implementation(libs.androidx.lifecycle.runtime.ktx)
    // For biometric requests for secure storage
    implementation(libs.androidx.biometric.ktx)

    // For Coroutines support
    implementation(libs.coroutines.android)

    // For Serialization extensions
    implementation(libs.kotlinx.serialization.json)

    // For Network support
    implementation(libs.okhttp)
    implementation(libs.okhttp.coroutines)

    // For Storage implementation
    implementation(libs.androidx.datastore.preferences)

    // For logging
    implementation(libs.timber)

    // Unit tests
    testImplementation(libs.jna)
    testImplementation(libs.junit)
    testImplementation(libs.junit.params)
    testImplementation(libs.mockk)
    testImplementation(libs.coroutines.test)
    testImplementation(libs.turbine)
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")

    // Integration tests
    androidTestImplementation(libs.androidx.test.runner)
    androidTestImplementation(libs.androidx.test.rules)
    androidTestImplementation(libs.androidx.test.junit.ktx)
    androidTestImplementation(libs.coroutines.test)
    androidTestImplementation(libs.mockk.android)
    androidTestImplementation(libs.mockk.agent)
    androidTestImplementation(libs.okhttp.mock.web.server)
}

publishing {
    publications {
        android.buildTypes.forEach {
            val buildTypeVariant = it.name.replaceFirstChar(Char::uppercase)

            // Publishing the android library we just need to build the library from the release component
            register<MavenPublication>("android$buildTypeVariant") {
                groupId = "com.radixdlt.sargon"
                artifactId = "sargon-android"
                version = project.sargonVersion(it.isDebuggable)

                afterEvaluate {
                    from(components[it.name])
                }
            }

            // Publishing the desktop bins we need to run the `desktopJar` task. For more info check
            // the comments of that task.
            register<MavenPublication>("desktop$buildTypeVariant") {
                groupId = "com.radixdlt.sargon"
                artifactId = "sargon-desktop-bins"
                version = project.sargonVersion(it.isDebuggable)

                afterEvaluate {
                    artifact(tasks.getByName("desktopJar$buildTypeVariant"))
                }

                pom {
                    withXml {
                        val dependencies = asNode().appendNode("dependencies")

                        val jni = dependencies.appendNode("dependency")
                        jni.appendNode("groupId", "net.java.dev.jna")
                        jni.appendNode("artifactId", "jna")
                        jni.appendNode("version", libs.versions.jna.get())
                        jni.appendNode("scope", "runtime")
                    }
                }
            }
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

// buildCargoNdk(Debug/Release) require for the existence of local.properties file
// Since it is ignored, we have to create it on CI's workflow
tasks.register("prepareLocalProperties") {
    onlyIf {
        // Will only run when file does not exist. Will not affect local builds.
        !File("${rootDir}/local.properties").exists()
    }

    doLast {
        file("${rootDir}/local.properties").writeText(
            "local.properties=${System.getenv("ANDROID_HOME")}"
        )
    }

    tasks.getByName("preBuild").dependsOn(this)
}

afterEvaluate {
    val generateBindings = tasks.register("generateUniFFIBindings") {
        group = BasePlugin.BUILD_GROUP

        doLast {
            // Uniffi needs a binary library to generate the bindings
            // - If in a previous task an android binary is generated, we use that
            //   (the build is intended to be used in an android device)
            // - If no android binary is found, we need to find a desktop generated one that was built
            //   for the same arch as the one the current task was invoked
            //   (the build was intended to run unit tests, so no android binaries were built)
            // - If no binaries are found, we fail the build
            val hostTarget = project.currentTargetTriple()

            // Android binaries take priority
            val binaryFile = Files.walk(File("${buildDir}/generated/src").toPath())
                .filter { !Files.isDirectory(it) }
                .map { it.toString() }
                .filter { path ->
                    path.endsWith("libsargon_uniffi.so") || path.endsWith("libsargon_uniffi.dylib") || path.endsWith("libsargon_uniffi.dll")
                }
                .map { File(it) }
                .toList()
                .find {
                    it.absolutePath.contains("jniLibs") || it.parentFile.name == hostTarget.jnaName
                }
                ?.relativeTo(rootDir.parentFile) ?: error("Could not find library file to generate bindings")

            exec {
                workingDir = rootDir.parentFile
                commandLine(
                    "cargo", "run", 
                    "--locked",
                    "-p", "sargon-uniffi",
                    "--bin", "sargon-bindgen",
                    "generate", "--library", binaryFile.toString(),
                    "--language", "kotlin",
                    "--out-dir", "${buildDir}/generated/src/kotlin"
                )
            }
        }
    }
    tasks.getByName("buildCargoDesktopDebug").finalizedBy(generateBindings)
    tasks.getByName("buildCargoDesktopRelease").finalizedBy(generateBindings)

    val copyJniLibs = tasks.register("copyJniLibs") {
        doLast {
            copy {
                from("src/main/jniLibs")
                into("${layout.buildDirectory.get()}/generated/src/jniLibs")
            }
            delete("src/main/jniLibs")
        }

        finalizedBy(generateBindings)
    }
    tasks.getByName("buildCargoNdkDebug").finalizedBy(copyJniLibs)
    tasks.getByName("buildCargoNdkRelease").finalizedBy(copyJniLibs)

    tasks.getByName("testDebugUnitTest").dependsOn("buildCargoDesktopDebug")
    tasks.getByName("testReleaseUnitTest").dependsOn("buildCargoDesktopRelease")

    tasks.register("buildForLocalDev") {
        group = "publishing"

        doLast {
            println("✅ Library is published in maven local with version:")
            println(project.sargonVersion(true))
        }

        dependsOn(
            "publishAndroidDebugPublicationToMavenLocal",
            "publishDesktopDebugPublicationToMavenLocal"
        )
    }
}

// Task that copies externally built artifacts into resources directory
tasks.register("copyExternalArtifacts") {
    doFirst {
        DesktopTargetTriple.ciSupported.forEach { triple ->
            exec {
                commandLine("mkdir", "-p", "${layout.buildDirectory.get()}/generated/src/resources/${triple.jnaName}")
            }
        }
    }

    doLast {
        val rustProjectDir = projectDir.parentFile.parentFile
        val artifactsDir = File(rustProjectDir, "artifacts")

        val directoryTargets = artifactsDir.listFiles()?.mapNotNull { file ->
            val triple = DesktopTargetTriple.from(file.name) ?: return@mapNotNull null
            file to triple
        }.orEmpty()

        if (directoryTargets.isEmpty()) {
            error("No files found in ${artifactsDir.absolutePath}")
        }

        directoryTargets.forEach { target ->
            exec {
                workingDir = rustProjectDir
                commandLine(
                    "cp",
                    "${target.first.canonicalPath}/${target.second.binaryName}",
                    "${layout.buildDirectory.get()}/generated/src/resources/${target.second.jnaName}/${target.second.binaryName}"
                )
            }
        }
    }
}

tasks.register("cargoClean", Exec::class) {
    group = BasePlugin.BUILD_GROUP
    workingDir = rootDir.parentFile
    commandLine("cargo", "clean")
}

tasks.getByName("clean") {
    dependsOn("cargoClean")
}