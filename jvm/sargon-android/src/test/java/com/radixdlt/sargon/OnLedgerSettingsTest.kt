package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.default
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import com.radixdlt.sargon.newAssetsExceptionList
import com.radixdlt.sargon.newDepositorsAllowList

class OnLedgerSettingsTest {

    @Test
    fun testDefault() {
        assertEquals(
            OnLedgerSettings(thirdPartyDeposits = ThirdPartyDeposits(
                depositRule = DepositRule.ACCEPT_ALL,
                assetsExceptionList = newAssetsExceptionList(assetsExceptionList = emptyList()),
                depositorsAllowList = newDepositorsAllowList(depositorsAllowList = emptyList())
            )),
            OnLedgerSettings.default()
        )
    }

}