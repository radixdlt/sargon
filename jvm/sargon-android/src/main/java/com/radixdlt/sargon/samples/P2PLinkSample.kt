package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.P2pLink
import com.radixdlt.sargon.P2pLinks
import com.radixdlt.sargon.RadixConnectPassword
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.randomBagOfBytes

@UsesSampleValues
val P2pLink.Companion.sample: Sample<P2pLink>
    get() = object : Sample<P2pLink> {

        override fun invoke(): P2pLink = P2pLink(
            connectionPassword = RadixConnectPassword(Exactly32Bytes.init(randomBagOfBytes(32))),
            displayName = "Sample"
        )

        override fun other(): P2pLink = P2pLink(
            connectionPassword = RadixConnectPassword(Exactly32Bytes.init(randomBagOfBytes(32))),
            displayName = "Sample Other"
        )
    }