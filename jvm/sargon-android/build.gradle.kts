import com.radixdlt.cargo.desktop.currentTargetTriple
import com.radixdlt.cargo.toml.sargonVersion
import org.gradle.configurationcache.extensions.capitalized
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import java.nio.file.Files

plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.serialization)
    alias(libs.plugins.android.cargo.ndk)
    alias(libs.plugins.kotlin.kover)
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
        getByName("debug") {
            java.srcDir("${buildDir}/generated/src/debug/java")
        }
        getByName("release") {
            java.srcDir("${buildDir}/generated/src/release/java")
        }
    }
}

cargoNdk {
    targets = arrayListOf("arm64", "arm")
    module = "../"
    librariesNames = arrayListOf("libsargon.so")
}

tasks.withType<Test> {
    useJUnitPlatform()
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
    // Cannot use version catalogues for aar. For some reason when published to Maven,
    // the jna dependency cannot be resolved
    implementation("net.java.dev.jna:jna:5.13.0@aar")

    // For lifecycle callbacks
    implementation(libs.androidx.appcompat)
    implementation(libs.androidx.lifecycle.runtime.ktx)
    // For biometric requests for secure storage
    implementation(libs.androidx.biometric.ktx)

    // For Coroutines support
    implementation(libs.coroutines.android)

    // For Serialization extensions
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.3")

    // For Network support
    implementation(libs.okhttp)
    implementation(libs.okhttp.coroutines)

    // For Storage implementation
    implementation(libs.androidx.datastore.preferences)

    // For logging
    implementation(libs.timber)

    // Unit tests
    testImplementation(libs.junit)
    testImplementation(libs.junit.params)
    testImplementation(libs.mockk)
    testImplementation(libs.coroutines.test)
    testImplementation(libs.turbine)
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
    testRuntimeOnly(project(":sargon-desktop"))

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
        register<MavenPublication>("release") {
            groupId = "com.radixdlt.sargon"
            artifactId = "sargon-android"
            version = project.sargonVersion()

            afterEvaluate {
                from(components["release"])
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

android.libraryVariants.all {
    val buildType = name
    val buildTypeUpper = buildType.capitalized()

    val generateBindings = tasks.register("generate${buildTypeUpper}UniFFIBindings") {
        group = BasePlugin.BUILD_GROUP

        var binaryFile: File? = null
        doFirst {
            // Uniffi needs a binary library to generate the bindings
            // - If in a previous task an android binary is generated, we use that
            //   (the build is intended to be used in an android device)
            // - If no android binary is found, we need to find a desktop generated one that was built
            //   for the same arch as the one the current task was invoked
            //   (the build was intended to run unit tests, so no android binaries were built)
            // - If no binaries are found, we fail the build
            val hostTarget = project.currentTargetTriple()

            // Android binaries take priority
            val androidBinaryFile = Files.walk(File("${rootDir}/sargon-android/src/main").toPath())
                .filter { !Files.isDirectory(it) }
                .map { it.toString() }
                .filter { path -> path.endsWith("libsargon.so") }
                .map { File(it) }.toList().firstOrNull()

            binaryFile = androidBinaryFile ?: Files.walk(File("${rootDir}/sargon-desktop/src/main").toPath())
                // Desktop binaries are searched
                .filter { !Files.isDirectory(it) }
                .map { it.toString() }
                .filter { path ->
                    path.endsWith("libsargon.so") || path.endsWith("libsargon.dylib") || path.endsWith("libsargon.dll")
                }
                .map { File(it) }
                .toList()
                .find { file ->
                    file.parentFile.name == hostTarget.jnaName
                }
        }

        doLast {
            val file = binaryFile
                ?.relativeTo(rootDir.parentFile)
                ?: error("Could not find library file to generate bindings")

            exec {
                workingDir = rootDir.parentFile
                commandLine(
                    "cargo", "run",
                    "--features", "build-binary",
                    "--bin", "sargon-bindgen",
                    "generate", "--library", file.toString(),
                    "--language", "kotlin",
                    "--out-dir", "${buildDir}/generated/src/${buildType}/java"
                )
            }
        }
    }

    javaCompileProvider.get().dependsOn(generateBindings)
}

tasks.register("cargoClean") {
    group = BasePlugin.BUILD_GROUP
    doLast {
        exec {
            workingDir = rootDir.parentFile
            println("Cleaning for aarch64-linux-android")
            commandLine("cargo", "clean", "--target", "aarch64-linux-android")
        }
        exec {
            workingDir = rootDir.parentFile
            println("Cleaning for armv7-linux-androideabi")
            commandLine("cargo", "clean", "--target", "armv7-linux-androideabi")
        }
    }
}

tasks.getByName("clean") {
    dependsOn("cargoClean")
}