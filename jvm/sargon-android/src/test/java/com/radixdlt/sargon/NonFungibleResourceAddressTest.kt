package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class NonFungibleResourceAddressTest {

    @Test
    fun test() {
        val addressString = "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa"
        val nonFungibleResourceAddress = NonFungibleResourceAddress.init(validating = addressString)

        assertEquals(addressString, nonFungibleResourceAddress.string)
        assertEquals(NetworkId.MAINNET, nonFungibleResourceAddress.networkId)
    }

}