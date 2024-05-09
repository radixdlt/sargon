package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.Url
import com.radixdlt.sargon.annotation.KoverIgnore

class Gateways private constructor(
    private val array: IdentifiedArray<Url, Gateway>
) : IdentifiedArray<Url, Gateway> by array {

    constructor(gateways: List<Gateway>) : this(
        IdentifiedArrayImpl(
            elements = gateways,
            identifier = { it.url }
        )
    )

    constructor(vararg gateway: Gateway) : this(gateways = gateway.asList())

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as Gateways

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "Gateways(array=$array)"
    }

}

fun List<Gateway>.asIdentifiable() = Gateways(gateways = this)

