package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class GatewaysTest: SampleTestable<Gateways> {

    override val samples: List<Sample<Gateways>>
        get() = listOf(Gateways.sample)

    @Test
    fun testNew() {
        val mainnet = Gateway.sampleMainnet()
        assertEquals(Gateway.sampleMainnet(), mainnet)
        val gateways = Gateways.init(current = mainnet)
        assertEquals(NetworkId.MAINNET, gateways.current.network.id)
    }

}