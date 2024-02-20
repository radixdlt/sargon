import org.jetbrains.kotlin.ir.backend.js.compile

plugins {
    alias(libs.plugins.kotlin.jvm)
}

dependencies {
    implementation(libs.jna)
}