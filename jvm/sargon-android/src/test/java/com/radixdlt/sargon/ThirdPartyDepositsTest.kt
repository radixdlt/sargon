package com.radixdlt.sargon

//class ThirdPartyDepositsTest {
//
//    @Test
//    fun testDefault() {
//        assertEquals(
//            ThirdPartyDeposits(
//                depositRule = DepositRule.ACCEPT_ALL,
//                assetsExceptionList = AssetsExceptionList.init(),
//                depositorsAllowList = DepositorsAllowList.init()
//            ),
//            ThirdPartyDeposits.default()
//        )
//    }
//
//    @Test
//    fun testAccountRecovery() {
//        assertEquals(
//            ThirdPartyDeposits(
//                depositRule = DepositRule.ACCEPT_ALL,
//                assetsExceptionList =  null,
//                depositorsAllowList = null
//            ),
//            ThirdPartyDeposits.accountRecoveryScanned()
//        )
//
//        assertEquals(
//            ThirdPartyDeposits(
//                depositRule = DepositRule.DENY_ALL,
//                assetsExceptionList =  null,
//                depositorsAllowList = null
//            ),
//            ThirdPartyDeposits.accountRecoveryScanned(depositRule = DepositRule.DENY_ALL)
//        )
//    }
//
//}