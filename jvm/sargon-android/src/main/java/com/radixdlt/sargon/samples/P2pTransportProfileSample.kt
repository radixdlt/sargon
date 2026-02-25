package com.radixdlt.sargon.samples

import com.radixdlt.sargon.P2pTransportProfile
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newP2pTransportProfileSample
import com.radixdlt.sargon.newP2pTransportProfileSampleOther

@UsesSampleValues
val P2pTransportProfile.Companion.sample: Sample<P2pTransportProfile>
    get() = object : Sample<P2pTransportProfile> {

        override fun invoke(): P2pTransportProfile = newP2pTransportProfileSample()

        override fun other(): P2pTransportProfile = newP2pTransportProfileSampleOther()
    }
