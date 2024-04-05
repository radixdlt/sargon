package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.isFungible
import com.radixdlt.sargon.extensions.isNonFungible
import com.radixdlt.sargon.extensions.isXRD
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.xrd
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class ResourceAddressTest: SampleTestable<ResourceAddress> {

    override val samples: List<Sample<ResourceAddress>>
        get() = listOf(ResourceAddress.sampleMainnet, ResourceAddress.sampleStokenet)

    @Test
    fun test() {
        val xrdAddressString = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
        val xrdAddress = ResourceAddress.init(validatingAddress = xrdAddressString)
        assertEquals(xrdAddressString, xrdAddress.string)
        assertEquals(NetworkId.MAINNET, xrdAddress.networkId)
        assertEquals(xrdAddress, ResourceAddress.xrd(NetworkId.MAINNET))
        assertTrue(xrdAddress.isFungible)
        assertFalse(xrdAddress.isNonFungible)

        val nonFungibleResourceAddress = ResourceAddress.init(
            validatingAddress = NonFungibleResourceAddress.sampleMainnet().string
        )
        assertTrue(nonFungibleResourceAddress.isNonFungible)
        assertFalse(nonFungibleResourceAddress.isFungible)
    }

    @Test
    fun testFormat() {
        val addressString = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
        val address = ResourceAddress.init(validatingAddress = addressString)

        assertEquals("reso...radxrd", address.formatted())
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
    fun testIsXrd() {
        assertTrue(ResourceAddress.sampleMainnet.xrd.isXRD)
        assertFalse(ResourceAddress.sampleMainnet.candy.isXRD)
    }

    @Test
    fun testAsGeneral() {
        val address = ResourceAddress.sampleMainnet()

        assertEquals(Address.Resource(address), address.asGeneral())
    }

}