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
}

cargoDesktop {
    buildType = BuildType.DEBUG
}

