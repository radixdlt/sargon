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
}

cargoNdk {
    module = "../sargon"  // Directory containing Cargo.toml
    librariesNames = arrayListOf("libsargon.so")
}

dependencies {
    implementation(libs.jna) {
        artifact {
            type = "aar"
        }
    }
}

sealed interface TargetTriple {
    val name: String
    val rustTarget: String
    val nativeTarget: String

    val linkerPath: String
    val ccPath: String
    val arPath: String

    fun environment(ndkHome: String): Map<String, String> {
        val linkerVar = rustTarget.uppercase().replace("-", "_")
        return mapOf(
            "CARGO_TARGET_${linkerVar}_LINKER" to "$ndkHome/$linkerPath",
            "CC" to "$ndkHome/$ccPath",
            "AR" to "$ndkHome/$arPath"
        )
    }

    object Aarch64 : TargetTriple {
        override val name: String = "Aarch64"
        override val rustTarget: String = "aarch64-linux-android"
        override val nativeTarget: String = "arm64-v8a"
        override val linkerPath: String =
            "toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang"
        override val ccPath: String =
            "toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang"
        override val arPath: String = "toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar"
    }

    object ArmeabiV7a : TargetTriple {
        override val name: String = "ArmeabiV7a"
        override val rustTarget: String = "armv7-linux-androideabi"
        override val nativeTarget: String = "armeabi-v7a"
        override val linkerPath: String =
            "toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi21-clang"
        override val ccPath: String =
            "toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi21-clang"
        override val arPath: String = "toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar"
    }

    companion object {
        val all: List<TargetTriple> = listOf(Aarch64, ArmeabiV7a)
    }
}

android.libraryVariants.all {
    val variantName = this.name
    val isDebug = variantName == "debug"

    // Register compile tasks
    TargetTriple.all.forEach { triple ->
        tasks.register("compile${variantName.capitalized()}${triple.name}") {
            onlyIf {
                (System.getenv("SDKROOT") != null).also {
                    if (!it) System.err.println("No 'SDKROOT' env found")
                }
            }
            onlyIf {
                (System.getenv("ANDROID_NDK_HOME") != null).also {
                    if (!it) System.err.println("No 'ANDROID_NDK_HOME' env found")
                }
            }

            doLast {
                exec {
                    workingDir = rootDir.parentFile
                    environment(triple.environment(System.getenv("ANDROID_NDK_HOME")))
                    commandLine(
                        "cargo",
                        "build",
                        if (isDebug) "" else "--release",
                        "--target",
                        triple.rustTarget
                    )
                }
                exec {
                    workingDir = rootDir
                    commandLine("mkdir", "-p", "sargon/src/main/jniLibs/${triple.nativeTarget}")
                }
                exec {
                    workingDir = rootDir.parentFile
                    commandLine(
                        "cp",
                        "target/${triple.rustTarget}/$variantName/libsargon.so",
                        "android/sargon/src/main/jniLibs/${triple.nativeTarget}/libsargon.so"
                    )
                }
            }
        }
    }
}

tasks.register("generateBindings") {
    val aarch64Binary =
        File("$rootDir/sargon/src/main/jniLibs/${TargetTriple.Aarch64.nativeTarget}/libsargon.so")
    val armeabiv7aBinary =
        File("$rootDir/sargon/src/main/jniLibs/${TargetTriple.ArmeabiV7a.nativeTarget}/libsargon.so")

    val binary = when {
        aarch64Binary.exists() -> aarch64Binary
        armeabiv7aBinary.exists() -> armeabiv7aBinary
        else -> null
    }

    onlyIf {
        (binary != null).also {
            if (!it) System.err.println("Missing .so library in jniLibs directory")
        }
    }

    exec {
        workingDir = rootDir.parentFile
        commandLine(
            "cargo", "run", "--bin", "uniffi-bindgen", "generate", "--library",
            binary!!.toRelativeString(workingDir), "--language", "kotlin", "--out-dir",
            "android/sargon/src/main/java"
        )
    }
}