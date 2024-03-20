package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.isLegacy
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.shortFormat
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class ComponentAddressTest {

    @Test
    fun test() {
        val addressString = "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"
        val componentAddress = ComponentAddress.init(validatingAddress = addressString)

        assertEquals(addressString, componentAddress.string)
        assertEquals(NetworkId.MAINNET, componentAddress.networkId)
    }

}