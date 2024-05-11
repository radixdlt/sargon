package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.PublicKeyHash
import com.radixdlt.sargon.P2pLink
import com.radixdlt.sargon.annotation.KoverIgnore

class P2pLinks private constructor(
    private val array: IdentifiedArray<PublicKeyHash, P2pLink>
) : IdentifiedArray<PublicKeyHash, P2pLink> by array {

    constructor(p2pLinks: List<P2pLink>) : this(
        IdentifiedArrayImpl(
            elements = p2pLinks,
            identifier = { it.id }
        )
    )

    constructor(vararg p2pLink: P2pLink) : this(p2pLinks = p2pLink.asList())

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as P2pLinks

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "P2pLinks(array=$array)"
    }

}

fun List<P2pLink>.asIdentifiable() = P2pLinks(p2pLinks = this)