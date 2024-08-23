package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.ResourceAppPreference
import com.radixdlt.sargon.ResourceIdentifier
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.resourcePreferencesGetHiddenResources
import com.radixdlt.sargon.resourcePreferencesHideResource
import com.radixdlt.sargon.resourcePreferencesUnhideResource

class ResourceAppPreferences private constructor(
    private val array: IdentifiedArray<ResourceIdentifier, ResourceAppPreference>
) : IdentifiedArray<ResourceIdentifier, ResourceAppPreference> by array {

    constructor(resourceAppPreferences: List<ResourceAppPreference>) : this(
        IdentifiedArrayImpl(
            elements = resourceAppPreferences,
            identifier = { it.resource }
        )
    )

    constructor(vararg resourceAppPreference: ResourceAppPreference) : this(resourceAppPreferences = resourceAppPreference.asList())

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as ResourceAppPreferences

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "ResourceAppPreferences(array=$array)"
    }

    companion object

}

fun List<ResourceAppPreference>.asIdentifiable() = ResourceAppPreferences(resourceAppPreferences = this)

val ResourceAppPreferences.hiddenResources
    get() = resourcePreferencesGetHiddenResources(asList())

fun ResourceAppPreferences.hideResource(resourceIdentifier: ResourceIdentifier) = resourcePreferencesHideResource(asList(), resourceIdentifier).asIdentifiable()

fun ResourceAppPreferences.unhideResource(resourceIdentifier: ResourceIdentifier) = resourcePreferencesUnhideResource(asList(), resourceIdentifier).asIdentifiable()