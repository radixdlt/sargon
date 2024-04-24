package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.accountRecoveryScanned
import com.radixdlt.sargon.extensions.default
import com.radixdlt.sargon.extensions.init
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ThirdPartyDepositsTest {

    @Test
    fun testDefault() {
        assertEquals(
            ThirdPartyDeposits(
                depositRule = DepositRule.ACCEPT_ALL,
                assetsExceptionList = AssetsExceptionList.init(),
                depositorsAllowList = DepositorsAllowList.init()
            ),
            ThirdPartyDeposits.default()
        )
    }

    @Test
    fun testAccountRecovery() {
        assertEquals(
            ThirdPartyDeposits(
                depositRule = DepositRule.ACCEPT_ALL,
                assetsExceptionList =  null,
                depositorsAllowList = null
            ),
            ThirdPartyDeposits.accountRecoveryScanned()
        )

        assertEquals(
            ThirdPartyDeposits(
                depositRule = DepositRule.DENY_ALL,
                assetsExceptionList =  null,
                depositorsAllowList = null
            ),
            ThirdPartyDeposits.accountRecoveryScanned(depositRule = DepositRule.DENY_ALL)
        )
    }

}