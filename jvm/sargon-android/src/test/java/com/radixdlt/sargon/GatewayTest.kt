package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.forNetwork
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.isWellKnown
import com.radixdlt.sargon.extensions.mainnet
import com.radixdlt.sargon.extensions.stokenet
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.wellKnown
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class GatewayTest {

    @Test
    fun testIsWellKnown() {
        assertTrue(Gateway.mainnet.isWellKnown)
        assertTrue(Gateway.stokenet.isWellKnown)
        assertFalse(Gateway.forNetwork(networkId = NetworkId.HAMMUNET).isWellKnown)

        assertEquals(
            listOf(Gateway.mainnet, Gateway.stokenet),
            Gateway.wellKnown
        )

    }

    @Test
    fun testNewUrlNetworkId() {
        assertEquals(
            Gateway.mainnet,
            Gateway.init(url = "https://mainnet.radixdlt.com", networkId = NetworkId.MAINNET)
        )

        assertEquals(
            Gateway.init(url = "https://mainnet.radixdlt.com/", networkId = NetworkId.MAINNET),
            Gateway.init(url = "https://mainnet.radixdlt.com", networkId = NetworkId.MAINNET)
        )
    }

    @Test
    fun testString() {
        val newGateway = Gateway.init(
            url = "https://hammunet-gateway.radixdlt.com",
            networkId = NetworkId.HAMMUNET
        )

        assertEquals(
            "https://hammunet-gateway.radixdlt.com/",
            newGateway.string
        )
    }

}