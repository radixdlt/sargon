package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AssetAddress
import com.radixdlt.sargon.AssetPreference
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.assetPreferencesGetHiddenAssets
import com.radixdlt.sargon.assetPreferencesHideAsset
import com.radixdlt.sargon.assetPreferencesUnhideAsset

class AssetPreferences private constructor(
    private val array: IdentifiedArray<AssetAddress, AssetPreference>
) : IdentifiedArray<AssetAddress, AssetPreference> by array {

    constructor(assetPreferences: List<AssetPreference>) : this(
        IdentifiedArrayImpl(
            elements = assetPreferences,
            identifier = { it.assetAddress }
        )
    )

    constructor(vararg assetPreference: AssetPreference) : this(assetPreferences = assetPreference.asList())

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as AssetPreferences

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "AssetPreferences(array=$array)"
    }

    companion object

}

fun List<AssetPreference>.asIdentifiable() = AssetPreferences(assetPreferences = this)

val AssetPreferences.hiddenAssets
    get() = assetPreferencesGetHiddenAssets(asList())

fun AssetPreferences.hideAsset(assetAddress: AssetAddress) = assetPreferencesHideAsset(asList(), assetAddress).asIdentifiable()

fun AssetPreferences.unhideAsset(assetAddress: AssetAddress) = assetPreferencesUnhideAsset(asList(), assetAddress).asIdentifiable()