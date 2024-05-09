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

dependencies {
    implementation("org.tomlj:tomlj:1.1.1")
}

plugins {
    `kotlin-dsl`
}