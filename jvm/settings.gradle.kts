pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}
dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
        maven {
            name = "GitHubPackages"
            url = uri("https://maven.pkg.github.com/radixdlt/maven-pkg")
            credentials {
                username = System.getenv("GPR_USER")
                password = System.getenv("GPR_TOKEN")
            }
        }
    }
}

rootProject.name = "Sargon JVM"
include(":android")
project(":android").projectDir = file("../examples/android")
include(":sargon-android")