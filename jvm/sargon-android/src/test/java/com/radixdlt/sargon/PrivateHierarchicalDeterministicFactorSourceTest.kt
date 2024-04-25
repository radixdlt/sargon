package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.babylon
import com.radixdlt.sargon.extensions.kind
import com.radixdlt.sargon.extensions.olympia
import com.radixdlt.sargon.extensions.supportsBabylon
import com.radixdlt.sargon.extensions.supportsOlympia
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class PrivateHierarchicalDeterministicFactorSourceTest {

    @Test
    fun testNewBabylon() {
        val sut = PrivateHierarchicalDeterministicFactorSource.babylon(
            isMain = true,
            mnemonicWithPassphrase = MnemonicWithPassphrase.sample()
        )
        Assertions.assertTrue(sut.factorSource.asGeneral().supportsBabylon)
    }

    @Test
    fun testNewOlympia() {
        val sut = PrivateHierarchicalDeterministicFactorSource.olympia(
            mnemonicWithPassphrase = MnemonicWithPassphrase.sample()
        )
        Assertions.assertTrue(sut.factorSource.asGeneral().supportsOlympia)
    }

    @Test
    fun testKindIsDevice() {
        assertEquals(
            FactorSourceKind.DEVICE,
            PrivateHierarchicalDeterministicFactorSource.babylon(
                isMain = true,
                mnemonicWithPassphrase = MnemonicWithPassphrase.sample()
            ).factorSource.kind
        )

        assertEquals(
            FactorSourceKind.DEVICE,
            PrivateHierarchicalDeterministicFactorSource.olympia(
                mnemonicWithPassphrase = MnemonicWithPassphrase.sample()
            ).factorSource.kind
        )
    }

}