package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class NonFungibleResourceAddressTest : SampleTestable<NonFungibleResourceAddress> {

    override val samples: List<Sample<NonFungibleResourceAddress>>
        get() = listOf(
            NonFungibleResourceAddress.sampleMainnet,
            NonFungibleResourceAddress.sampleStokenet
        )

    @Test
    fun test() {
        val addressString = "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa"
        val nonFungibleResourceAddress = NonFungibleResourceAddress.init(validating = addressString)

        assertEquals(addressString, nonFungibleResourceAddress.string)
        assertEquals(NetworkId.MAINNET, nonFungibleResourceAddress.networkId)
    }

    @Test
    fun testFormat() {
        val addressString = "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa"
        val nonFungibleResourceAddress = NonFungibleResourceAddress.init(validating = addressString)

        assertEquals("reso...c9wlxa", nonFungibleResourceAddress.formatted())
        assertEquals(
            addressString,
            nonFungibleResourceAddress.formatted(format = AddressFormat.FULL)
        )
        assertEquals(
            addressString,
            nonFungibleResourceAddress.formatted(format = AddressFormat.RAW)
        )
    }
}