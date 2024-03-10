gradlePlugin {
    plugins {
        create("cargo-desktop") {
            id = "com.radixdlt.cargo.desktop"
            implementationClass = "com.radixdlt.cargo.desktop.CargoDesktopPlugin"
        }
    }
}

repositories {
    mavenCentral()
}

plugins {
    `kotlin-dsl`
}