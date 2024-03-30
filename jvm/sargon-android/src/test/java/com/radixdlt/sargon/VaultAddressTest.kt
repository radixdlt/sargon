package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.isFungible
import com.radixdlt.sargon.extensions.isNonFungible
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class VaultAddressTest: SampleTestable<VaultAddress> {

    override val samples: List<Sample<VaultAddress>>
        get() = listOf(VaultAddress.sampleMainnet, VaultAddress.sampleStokenet)

    @Test
    fun test() {
        val addressString = "internal_vault_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq"
        val vaultAddress = VaultAddress.init(validatingAddress = addressString)

        assertEquals(addressString, vaultAddress.string)
        assertEquals(NetworkId.MAINNET, vaultAddress.networkId)
        assertTrue(vaultAddress.isFungible)
        assertFalse(vaultAddress.isNonFungible)
    }

    @Test
    fun testFormat() {
        val addressString = "internal_vault_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq"
        val address = VaultAddress.init(validatingAddress = addressString)

        assertEquals("inte...ju49fq", address.formatted())
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