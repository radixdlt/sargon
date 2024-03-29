import com.radixdlt.cargo.desktop.BuildType

plugins {
    id("java-library")
    id("maven-publish")
    id("com.radixdlt.cargo.desktop")
}

java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}

dependencies {
    implementation("net.java.dev.jna:jna:5.13.0")
}

publishing {
    publications {
        register<MavenPublication>("release") {
            groupId = "com.radixdlt.sargon"
            artifactId = "sargon-desktop-bins"
            version = System.getenv("SARGON_VERSION")

            from(components["java"])
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

cargoDesktop {
    buildType = BuildType.RELEASE
}

