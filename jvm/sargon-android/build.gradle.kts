import org.gradle.configurationcache.extensions.capitalized

plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.android.cargo.ndk)
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

dependencies {
    // Cannot use version catalogues here. For some reason when published to Maven,
    // the jna dependency cannot be resolved
    implementation("net.java.dev.jna:jna:5.13.0@aar")
    implementation("androidx.annotation:annotation:1.7.1")
    implementation("androidx.compose.ui:ui-tooling-preview-android:1.6.2")

    testRuntimeOnly(project(":sargon-desktop-debug"))
    testImplementation(libs.junit)
}

publishing {
    publications {
        register<MavenPublication>("release") {
            groupId = "com.radixdlt.sargon"
            artifactId = "sargon-android"
            version = System.getenv("SARGON_VERSION")

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
            "cargo", "run", "--bin", "uniffi-bindgen", "generate", "--library",
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