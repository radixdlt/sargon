package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.isFungible
import com.radixdlt.sargon.extensions.isNonFungible
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.xrd
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class ResourceAddressTest {

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

}