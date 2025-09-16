package com.radixdlt.cargo.desktop

enum class BuildType {
    DEBUG,
    RELEASE;

    fun isRelease() = this == RELEASE

    val lowercase: String
        get() = name.lowercase()

    val capitalised: String
        get() = lowercase.replaceFirstChar(Char::uppercase)

    companion object {
        fun from(property: String?) = property?.lowercase()?.let { propertyLowercased ->
            BuildType.values().find { it.lowercase == propertyLowercased }
        }
    }
}