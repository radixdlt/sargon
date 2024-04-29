package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.all
import com.radixdlt.sargon.extensions.changeCurrent
import com.radixdlt.sargon.extensions.default
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.mainnet
import com.radixdlt.sargon.extensions.stokenet
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class GatewaysTest : SampleTestable<Gateways> {

    override val samples: List<Sample<Gateways>>
        get() = listOf(Gateways.sample)

    @Test
    fun testNew() {
        val mainnet = Gateway.sampleMainnet()
        assertEquals(Gateway.sampleMainnet(), mainnet)
        val gateways = Gateways.init(current = mainnet)
        assertEquals(NetworkId.MAINNET, gateways.current.network.id)
    }

    @Test
    fun testDefault() {
        assertEquals(
            Gateways(
                current = Gateway.mainnet,
                other = OtherGateways.init(Gateway.stokenet)
            ),
            Gateways.default
        )
    }

    @Test
    fun testChangeCurrent() {
        val newGateway = Gateway.init(
            url = "https://hammunet-gateway.radixdlt.com",
            networkId = NetworkId.HAMMUNET
        )
        val gateways = Gateways.default.changeCurrent(newCurrent = newGateway)

        assertEquals(
            Gateways(
                current = newGateway,
                other = OtherGateways.init(Gateway.stokenet, Gateway.mainnet)
            ),
            gateways
        )

        assertEquals(
            listOf(newGateway, Gateway.stokenet, Gateway.mainnet),
            gateways.all
        )
    }
}