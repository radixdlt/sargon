import org.gradle.configurationcache.extensions.capitalized

plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.android.cargo.ndk)
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

dependencies {
    implementation(libs.jna) {
        artifact {
            type = "aar"
        }
    }
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