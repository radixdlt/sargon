package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AssetException
import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.annotation.KoverIgnore

class AssetsExceptionList private constructor(
    private val array: IdentifiedArray<ResourceAddress, AssetException>
) : IdentifiedArray<ResourceAddress, AssetException> by array {

    constructor(assetExceptions: List<AssetException>) : this(
        IdentifiedArrayImpl(
            elements = assetExceptions,
            identifier = { it.address }
        )
    )

    constructor(
        vararg assetException: AssetException
    ) : this(assetExceptions = assetException.asList())

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as AssetsExceptionList

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "AssetsExceptionList(array=$array)"
    }

}

fun List<AssetException>.asIdentifiable() = AssetsExceptionList(assetExceptions = this)