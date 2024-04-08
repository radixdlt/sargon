import com.radixdlt.cargo.desktop.BuildType

plugins {
    id("java-library")
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

cargoDesktop {
    buildType = BuildType.DEBUG
}

