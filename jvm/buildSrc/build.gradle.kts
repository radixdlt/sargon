gradlePlugin {
    plugins {
        create("cargo-desktop") {
            id = "com.radixdlt.cargo.desktop"
            implementationClass = "com.radixdlt.cargo.desktop.DesktopCargoPlugin"
        }
    }
}

repositories {
    mavenCentral()
}

plugins {
    `kotlin-dsl`
}