package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class AccessControllerAddressTest: SampleTestable<AccessControllerAddress> {

    override val samples: List<Sample<AccessControllerAddress>>
        get() = listOf(AccessControllerAddress.sampleMainnet, AccessControllerAddress.sampleStokenet)

    @Test
    fun test() {
        val bech32 = "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a"
        val address = AccessControllerAddress.init(validatingAddress = bech32)

        assertEquals(bech32, address.string)
        assertEquals(NetworkId.MAINNET, address.networkId)
        assertThrows<CommonException.FailedToFindNetworkIdFromBech32mString> {
            AccessControllerAddress.init(validatingAddress = "just text")
        }
    }

    @Test
    fun testFormat() {
        val addressString = "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a"
        val address = AccessControllerAddress.init(validatingAddress = addressString)

        assertEquals("acce...yq6y5a", address.formatted())
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
    fun testAsGeneral() {
        val address = AccessControllerAddress.sampleMainnet()

        assertEquals(Address.AccessController(address), address.asGeneral())
    }

}