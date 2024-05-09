package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.ResourceOrNonFungible
import com.radixdlt.sargon.annotation.KoverIgnore

class DepositorsAllowList private constructor(
    private val array: IdentifiedArray<ResourceOrNonFungible, ResourceOrNonFungible>
) : IdentifiedArray<ResourceOrNonFungible, ResourceOrNonFungible> by array {

    constructor(resourcesOrNonFungibles: List<ResourceOrNonFungible>) : this(
        IdentifiedArrayImpl(
            elements = resourcesOrNonFungibles,
            identifier = { it }
        )
    )

    constructor(vararg resourceOrNonFungible: ResourceOrNonFungible) : this(
        resourcesOrNonFungibles = resourceOrNonFungible.asList()
    )

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as DepositorsAllowList

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "DepositorsAllowList(array=$array)"
    }

}

fun List<ResourceOrNonFungible>.asIdentifiable() =
    DepositorsAllowList(resourcesOrNonFungibles = this)