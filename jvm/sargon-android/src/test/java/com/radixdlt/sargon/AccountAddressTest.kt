package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.isLegacy
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.shortFormat
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class AccountAddressTest: SampleTestable<AccountAddress> {

    override val samples: List<Sample<AccountAddress>>
        get() = listOf(AccountAddress.sampleMainnet, AccountAddress.sampleStokenet)

    @Test
    fun test() {
        val bech32 = "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm"
        val key = PublicKey.Ed25519.init(
            hex = "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d"
        )

        val address0 = AccountAddress.init(
            publicKey = key,
            networkId = NetworkId.MAINNET
        )
        assertEquals(bech32, address0.string)

        val address1 = AccountAddress.init(validatingAddress = bech32)
        assertEquals(bech32, address1.string)
        assertEquals("acco...m2kzdm", address1.shortFormat)
        assertEquals(NetworkId.MAINNET, address1.networkId)
        assertFalse(address0.isLegacy)
    }

}