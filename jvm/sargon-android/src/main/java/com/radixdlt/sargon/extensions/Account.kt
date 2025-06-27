package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Account
import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.HierarchicalDeterministicFactorInstance
import com.radixdlt.sargon.accountIsLegacy
import com.radixdlt.sargon.accountUnsecuredControllingFactorInstance

val Account.isLegacy: Boolean
    get() = accountIsLegacy(account = this)

val Account.isUnsecuredLedgerControlled: Boolean
    get() {
        val unsecuredTransactionSigningFactorSourceId = unsecuredControllingFactorInstance
            ?.factorSourceId

        return if (unsecuredTransactionSigningFactorSourceId != null) {
            unsecuredTransactionSigningFactorSourceId.kind ==
                    FactorSourceKind.LEDGER_HQ_HARDWARE_WALLET
        } else {
            false
        }
    }

val Account.unsecuredControllingFactorInstance: HierarchicalDeterministicFactorInstance?
    get() = accountUnsecuredControllingFactorInstance(account = this)