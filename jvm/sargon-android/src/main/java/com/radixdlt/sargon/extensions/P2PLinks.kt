package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.IdentifiedArray
import com.radixdlt.sargon.IdentifiedArrayImpl
import com.radixdlt.sargon.P2pLink

class P2pLinks private constructor(
    array: IdentifiedArray<Hash, P2pLink>
) : IdentifiedArray<Hash, P2pLink> by array {

    constructor(p2pLinks: List<P2pLink>) : this(
        IdentifiedArrayImpl(
            elements = p2pLinks,
            identifier = { it.id }
        )
    )

    constructor(vararg p2pLink: P2pLink) : this(p2pLinks = p2pLink.asList())
}