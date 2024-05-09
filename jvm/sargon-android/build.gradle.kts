import org.gradle.configurationcache.extensions.capitalized
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import java.io.ByteArrayOutputStream

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
            annotatedBy("com.radixdlt.sargon.annotation.KoverIgnore")
        }
        includes {
            packages("com.radixdlt.sargon.extensions")
            packages("com.radixdlt.sargon.antenna")
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

    // For Coroutines support
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.8.0")

    // For Serialization extensions
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.3")

    // For Network support
    implementation(platform("com.squareup.okhttp3:okhttp-bom:5.0.0-alpha.12"))
    implementation("com.squareup.okhttp3:okhttp")
    implementation("com.squareup.okhttp3:okhttp-coroutines")

    testImplementation(libs.junit)
    testImplementation(libs.mockk)
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
    testDebugRuntimeOnly(project(":sargon-desktop-debug"))
    testReleaseRuntimeOnly(project(":sargon-desktop-release"))
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
            artifactId = "sargon-android"

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

    val generateBindings = tasks.register(
        "generate${buildTypeUpper}UniFFIBindings",
        Exec::class
    ) {
        group = BasePlugin.BUILD_GROUP

        workingDir = rootDir.parentFile
        commandLine(
            "cargo", "run", "--features", "build-binary", "--bin", "sargon-bindgen", "generate", "--library",
            "${rootDir}/${project.name}/src/main/jniLibs/arm64-v8a/libsargon.so", "--language", "kotlin",
            "--out-dir", "${buildDir}/generated/src/${buildType}/java"
        )

        dependsOn("buildCargoNdk${buildTypeUpper}")
    }

    javaCompileProvider.get().dependsOn(generateBindings)

    // Some stuff here is broken, since Android Tests don't run after running gradle build,
    // but do otherwise. Also CI is funky.
    tasks.named("compile${buildTypeUpper}Kotlin").configure {
        dependsOn(generateBindings)
    }

    tasks.named("connectedDebugAndroidTest").configure {
        dependsOn(generateBindings)
    }
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