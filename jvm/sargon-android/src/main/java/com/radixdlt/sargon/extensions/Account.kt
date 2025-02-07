package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Account
import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.HierarchicalDeterministicFactorInstance
import com.radixdlt.sargon.accountIsLegacyOlympia
import com.radixdlt.sargon.accountUnsecuredControllingFactorInstance

val Account.isLegacyOlympia: Boolean
    get() = accountIsLegacyOlympia(account = this)

val Account.isUnsecuredLedgerControlled: Boolean
    get() = unsecuredControllingFactorInstance?.factorSourceId?.kind ==
            FactorSourceKind.LEDGER_HQ_HARDWARE_WALLET

val Account.unsecuredControllingFactorInstance: HierarchicalDeterministicFactorInstance?
    get() = accountUnsecuredControllingFactorInstance(account = this)