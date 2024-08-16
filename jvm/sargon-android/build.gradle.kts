import com.radixdlt.cargo.toml.sargonVersion
import org.gradle.configurationcache.extensions.capitalized
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

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
    testDebugRuntimeOnly(project(":sargon-desktop-debug"))
    testReleaseRuntimeOnly(project(":sargon-desktop-release"))

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

    val generateBindings = tasks.register(
        "generate${buildTypeUpper}UniFFIBindings",
        Exec::class
    ) {
        val regenerate = properties["regenerate"] ?: true //TODO improve by using gradle outputs

        onlyIf {
            regenerate == true || !File("${buildDir}/generated/src/${buildType}/java").exists()
        }

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