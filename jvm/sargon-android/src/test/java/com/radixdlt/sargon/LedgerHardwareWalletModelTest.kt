package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.string
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class LedgerHardwareWalletModelTest {
    @Test
    fun testStringRoundtrip() {
        assertEquals(
            LedgerHardwareWalletModel.NANO_S_PLUS,
            LedgerHardwareWalletModel.init(LedgerHardwareWalletModel.NANO_S_PLUS.string)
        )
    }
}