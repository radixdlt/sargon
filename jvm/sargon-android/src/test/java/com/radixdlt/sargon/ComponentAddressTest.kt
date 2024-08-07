package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.isGlobal
import com.radixdlt.sargon.extensions.isInternal
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class ComponentAddressTest: SampleTestable<ComponentAddress> {

    override val samples: List<Sample<ComponentAddress>>
        get() = listOf(ComponentAddress.sampleMainnet, ComponentAddress.sampleStokenet)

    @Test
    fun test() {
        val addressString = "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"
        val componentAddress = ComponentAddress.init(validatingAddress = addressString)

        assertEquals(addressString, componentAddress.string)
        assertEquals(NetworkId.MAINNET, componentAddress.networkId)
    }

    @Test
    fun testFormat() {
        val addressString = "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"
        val address = ComponentAddress.init(validatingAddress = addressString)

        assertEquals("comp...faucet", address.formatted())
        assertEquals(
            addressString,
            address.formatted(format = AddressFormat.FULL)
        )
        assertEquals(
            addressString,
            address.formatted(format = AddressFormat.RAW)
        )
    }

    @Test
    fun testIsGlobal() {
        assertTrue(ComponentAddress.sampleMainnet().isGlobal)
        assertFalse(ComponentAddress.sampleMainnet.other().isGlobal)
    }

    @Test
    fun testIsInternal() {
        assertTrue(ComponentAddress.sampleMainnet.other().isInternal)
        assertFalse(ComponentAddress.sampleMainnet().isInternal)
    }

    @Test
    fun testAsGeneral() {
        val address = ComponentAddress.sampleMainnet()

        assertEquals(Address.Component(address), address.asGeneral())
    }

}