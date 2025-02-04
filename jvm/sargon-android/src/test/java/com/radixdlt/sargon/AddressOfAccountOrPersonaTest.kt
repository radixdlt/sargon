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
import org.junit.jupiter.api.assertThrows

class AddressOfAccountOrPersonaTest: SampleTestable<AddressOfAccountOrPersona> {

    override val samples: List<Sample<AddressOfAccountOrPersona>>
        get() = listOf(AddressOfAccountOrPersona.sampleMainnet, AddressOfAccountOrPersona.sampleStokenet)

    @Test
    fun testInit() {
        val accountAddress = "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"

        with(AddressOfAccountOrPersona.init(validating = accountAddress)) {
            assertEquals(accountAddress, string)
            assertEquals(NetworkId.MAINNET, networkId)
        }

        assertThrows<CommonException.AddressInvalidEntityType> {
            AddressOfAccountOrPersona.init(validating = PackageAddress.sampleMainnet().string)
        }
    }

    @Test
    fun testFormat() {
        val addressString = "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
        val address = AddressOfAccountOrPersona.init(validating = addressString)

        assertEquals("acco...nvjdwr", address.formatted())
        assertEquals(
            addressString,
            address.formatted(format = AddressFormat.FULL)
        )
        assertEquals(
            addressString,
            address.formatted(format = AddressFormat.RAW)
        )
    }

}