package com.radixdlt.sargon

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
        val accountAddress = "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"

        with(AddressOfAccountOrPersona.init(validating = accountAddress)) {
            assertEquals(accountAddress, string)
            assertEquals(NetworkId.MAINNET, networkId)
        }

        assertThrows<CommonException.FailedToDecodeAddressFromBech32> {
            AddressOfAccountOrPersona.init(validating = PackageAddress.sampleMainnet().string)
        }
    }

}