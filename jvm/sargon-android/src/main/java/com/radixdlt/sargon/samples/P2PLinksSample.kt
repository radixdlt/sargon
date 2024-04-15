package com.radixdlt.sargon.samples

import com.radixdlt.sargon.P2pLink
import com.radixdlt.sargon.P2pLinks
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.init

@UsesSampleValues
val P2pLinks.Companion.sample: Sample<P2pLinks>
    get() = object : Sample<P2pLinks> {

        override fun invoke(): P2pLinks = P2pLinks.init(
            P2pLink.sample(),
            P2pLink.sample.other(),
        )

        override fun other(): P2pLinks = P2pLinks.init(P2pLink.sample())
    }