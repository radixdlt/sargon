package com.radixdlt.sargon.samples

import com.radixdlt.sargon.P2pLink
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newP2pLinkSample
import com.radixdlt.sargon.newP2pLinkSampleOther

@UsesSampleValues
val P2pLink.Companion.sample: Sample<P2pLink>
    get() = object : Sample<P2pLink> {

        override fun invoke(): P2pLink = newP2pLinkSample()

        override fun other(): P2pLink = newP2pLinkSampleOther()
    }