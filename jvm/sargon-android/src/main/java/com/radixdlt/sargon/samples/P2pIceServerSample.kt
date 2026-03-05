package com.radixdlt.sargon.samples

import com.radixdlt.sargon.P2pStunServer
import com.radixdlt.sargon.P2pTurnServer
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newP2pStunServerSample
import com.radixdlt.sargon.newP2pStunServerSampleOther
import com.radixdlt.sargon.newP2pTurnServerSample
import com.radixdlt.sargon.newP2pTurnServerSampleOther

@UsesSampleValues
val P2pStunServer.Companion.sample: Sample<P2pStunServer>
    get() = object : Sample<P2pStunServer> {

        override fun invoke(): P2pStunServer = newP2pStunServerSample()

        override fun other(): P2pStunServer = newP2pStunServerSampleOther()
    }

@UsesSampleValues
val P2pTurnServer.Companion.sample: Sample<P2pTurnServer>
    get() = object : Sample<P2pTurnServer> {

        override fun invoke(): P2pTurnServer = newP2pTurnServerSample()

        override fun other(): P2pTurnServer = newP2pTurnServerSampleOther()
    }
