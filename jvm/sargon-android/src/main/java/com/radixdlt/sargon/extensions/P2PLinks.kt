package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.PublicKeyHash
import com.radixdlt.sargon.P2pLink
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.newP2PLinkFromJsonBytes
import com.radixdlt.sargon.newP2pLinksFromJsonBytes
import com.radixdlt.sargon.p2PLinkToJsonBytes
import com.radixdlt.sargon.p2pLinksToJsonBytes

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

    companion object

}

fun List<P2pLink>.asIdentifiable() = P2pLinks(p2pLinks = this)

@Throws(SargonException::class)
fun P2pLinks.Companion.fromJson(json: String) =
    newP2pLinksFromJsonBytes(jsonBytes = bagOfBytes(json))

fun P2pLinks.toJson(): String =
    p2pLinksToJsonBytes(asList()).string