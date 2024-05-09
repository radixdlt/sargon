package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.P2pLinks
import com.radixdlt.sargon.extensions.id
import com.radixdlt.sargon.samples.sample

internal class P2PLinksTest: IdentifiedArrayTest<P2pLinks, Hash, P2pLink>() {
    override fun element(): P2pLink = P2pLink.sample()

    override fun elementWithDifferentId(): P2pLink = P2pLink.sample.other()

    override fun identifier(element: P2pLink): Hash = element.id

    override fun init(element: P2pLink): P2pLinks = P2pLinks(element)
}