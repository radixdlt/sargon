package com.radixdlt.cargo.desktop

import org.gradle.configurationcache.extensions.capitalized

enum class BuildType {
    DEBUG,
    RELEASE;

    fun isRelease() = this == RELEASE

    val lowercase: String
        get() = name.lowercase()

    val capitalised: String
        get() = lowercase.capitalized()

    companion object {
        fun from(property: String?) = property?.lowercase()?.let { propertyLowercased ->
            BuildType.values().find { it.lowercase == propertyLowercased }
        }
    }
}