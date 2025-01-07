package com.radixdlt.sargon.os.signing

import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.PerFactorSourceInputOfAuthIntent
import com.radixdlt.sargon.PerFactorSourceInputOfSubintent
import com.radixdlt.sargon.PerFactorSourceInputOfTransactionIntent

data class PerFactorSourceInput<SP : Signable.Payload, ID : Signable.ID>(
    /**
     * The factor source which the interactor should request signatures with
     */
    val factorSourceId: FactorSourceIdFromHash,
    /**
     * A set of transactions to sign, with multiple derivations paths.
     */
    val perTransaction: List<TransactionSignRequestInput<SP>>,
    /**
     * A collection of transactions which would be invalid if the user skips
     * signing with this factor source.
     */
    val invalidTransactionsIfNeglected: List<InvalidTransactionIfNeglected<ID>>
)

internal fun PerFactorSourceInputOfTransactionIntent.into() = PerFactorSourceInput(
    factorSourceId = factorSourceId,
    perTransaction = perTransaction.map { it.into() },
    invalidTransactionsIfNeglected = invalidTransactionsIfNeglected.map { it.into() }
)

internal fun PerFactorSourceInputOfSubintent.into() = PerFactorSourceInput(
    factorSourceId = factorSourceId,
    perTransaction = perTransaction.map { it.into() },
    invalidTransactionsIfNeglected = invalidTransactionsIfNeglected.map { it.into() }
)

internal fun PerFactorSourceInputOfAuthIntent.into() = PerFactorSourceInput(
    factorSourceId = factorSourceId,
    perTransaction = perTransaction.map { it.into() },
    invalidTransactionsIfNeglected = invalidTransactionsIfNeglected.map { it.into() }
)