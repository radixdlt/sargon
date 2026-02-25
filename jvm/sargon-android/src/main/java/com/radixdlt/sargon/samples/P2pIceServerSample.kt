package com.radixdlt.sargon.samples

import com.radixdlt.sargon.P2pIceServer
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newP2pIceServerSample
import com.radixdlt.sargon.newP2pIceServerSampleOther

@UsesSampleValues
val P2pIceServer.Companion.sample: Sample<P2pIceServer>
    get() = object : Sample<P2pIceServer> {

        override fun invoke(): P2pIceServer = newP2pIceServerSample()

        override fun other(): P2pIceServer = newP2pIceServerSampleOther()
    }
