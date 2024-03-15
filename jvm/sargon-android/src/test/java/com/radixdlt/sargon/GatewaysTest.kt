package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

class GatewaysTest {

    @Test
    fun testEquals() {
        val a = Gateways.sample()
        val b = Gateways.sample.other()

        assertEquals(Gateways.sample(), a)
        assertNotEquals(a, b)
        assertEquals(Gateways.sample.other(), b)
    }

    @Test
    fun testHashCode() {
        val a = Gateways.sample()
        val b = Gateways.sample.other()
        assertEquals(1, setOf(a, a).size)
        assertEquals(1, setOf(b, b).size)
        assertEquals(2, setOf(a, b, b, a).size)
    }

    @Test
    fun testNew() {
        val mainnet = Gateway.sampleMainnet()
        assertEquals(Gateway.sampleMainnet(), mainnet)
        val gateways = Gateways.init(current = mainnet)
        assertEquals(NetworkId.MAINNET, gateways.current.network.id)
    }

}