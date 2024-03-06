package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ResourceAddressTest {

    @Test
    fun test() {
        val bech32 = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
        val address = ResourceAddress.init(validatingAddress = bech32)
        assertEquals(bech32, address.string)
        assertEquals(NetworkId.MAINNET, address.networkId)
    }

}