package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.babylon
import com.radixdlt.sargon.extensions.isMain
import com.radixdlt.sargon.extensions.kind
import com.radixdlt.sargon.extensions.olympia
import com.radixdlt.sargon.extensions.supportsBabylon
import com.radixdlt.sargon.extensions.supportsOlympia
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class FactorSourceTest : SampleTestable<FactorSource> {

    override val samples: List<Sample<FactorSource>>
        get() = listOf(FactorSource.sample)

    @Test
    fun testKind() {
        assertEquals(
            FactorSourceKind.DEVICE,
            FactorSource.sample().kind
        )

        assertEquals(
            FactorSourceKind.LEDGER_HQ_HARDWARE_WALLET,
            FactorSource.sample.other().kind
        )
    }

    @Test
    fun testNewBabylonIsMain() {
        assertTrue(
            FactorSource.Device.babylon(
                isMain = true,
                mnemonicWithPassphrase = MnemonicWithPassphrase.sample()
            ).isMain
        )
    }

    @Test
    fun testNewBabylonIsNotMain() {
        assertFalse(
            FactorSource.Device.babylon(
                isMain = false,
                mnemonicWithPassphrase = MnemonicWithPassphrase.sample()
            ).isMain
        )
    }

    @Test
    fun testNewBabylon() {
        val factorSource = FactorSource.Device.babylon(
            isMain = false,
            mnemonicWithPassphrase = MnemonicWithPassphrase.sample()
        )
        assertTrue(factorSource.supportsBabylon)
        assertFalse(factorSource.supportsOlympia)
    }

    @Test
    fun testNewOlympia() {
        val factorSource = FactorSource.Device.olympia(
            mnemonicWithPassphrase = MnemonicWithPassphrase.sample()
        )
        assertTrue(factorSource.supportsOlympia)
        assertFalse(factorSource.supportsBabylon)
    }
}