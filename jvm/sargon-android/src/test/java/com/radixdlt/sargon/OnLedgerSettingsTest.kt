package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.default
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class OnLedgerSettingsTest {

    @Test
    fun testDefault() {
        assertEquals(
            OnLedgerSettings(thirdPartyDeposits = ThirdPartyDeposits(
                depositRule = DepositRule.ACCEPT_ALL,
                assetsExceptionList = emptyList(),
                depositorsAllowList = emptyList()
            )),
            OnLedgerSettings.default()
        )
    }

}