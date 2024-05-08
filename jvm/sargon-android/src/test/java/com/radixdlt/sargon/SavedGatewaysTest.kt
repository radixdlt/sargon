package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.Gateways
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

class SavedGatewaysTest: SampleTestable<SavedGateways> {

    override val samples: List<Sample<SavedGateways>>
        get() = listOf(SavedGateways.sample)

    @Test
    fun testNew() {
        val mainnet = Gateway.sampleMainnet()
        assertEquals(Gateway.sampleMainnet(), mainnet)
        val gateways = SavedGateways.init(current = mainnet)
        assertEquals(NetworkId.MAINNET, gateways.current.network.id)
    }

    @Test
    fun testDefault() {
        assertEquals(
            SavedGateways(
                current = Gateway.mainnet,
                other = Gateways(Gateway.stokenet).asList()
            ),
            SavedGateways.default
        )
    }

    @Test
    fun testChangeCurrent() {
        val newGateway = Gateway.init(
            url = "https://hammunet-gateway.radixdlt.com",
            networkId = NetworkId.HAMMUNET
        )
        val gateways = SavedGateways.default.changeCurrent(newCurrent = newGateway)

        assertEquals(
            SavedGateways(
                current = newGateway,
                other = Gateways(Gateway.stokenet, Gateway.mainnet).asList()
            ),
            gateways
        )

        assertEquals(
            listOf(newGateway, Gateway.stokenet, Gateway.mainnet),
            gateways.all
        )
    }
}