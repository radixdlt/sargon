package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DepositRule
import com.radixdlt.sargon.ThirdPartyDeposits
import com.radixdlt.sargon.newThirdPartyDepositsDefault

fun ThirdPartyDeposits.Companion.default() = newThirdPartyDepositsDefault()

fun ThirdPartyDeposits.Companion.accountRecoveryScanned(
    depositRule: DepositRule = DepositRule.ACCEPT_ALL
) = ThirdPartyDeposits(
    depositRule = depositRule,
    assetsExceptionList = null,
    depositorsAllowList = null
)