package com.radixdlt.sargon.os.signing

import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.OwnedFactorInstance
import com.radixdlt.sargon.TransactionSignRequestInputOfAuthIntent
import com.radixdlt.sargon.TransactionSignRequestInputOfSubintent
import com.radixdlt.sargon.TransactionSignRequestInputOfTransactionIntent

data class TransactionSignRequestInput<SP: Signable.Payload> (
    /**
     * Payload to sign
     */
    val payload: SP,
    /**
     * ID of factor to use to sign
     */
    val factorSourceId: FactorSourceIdFromHash,
    /**
     * The derivation paths to use to derive the private keys to sign with. The
     * `factor_source_id` of each item must match `factor_source_id`.
     */
    val ownedFactorInstances: List<OwnedFactorInstance>
)

internal fun TransactionSignRequestInputOfTransactionIntent.into()
    = TransactionSignRequestInput(
        payload = Signable.Payload.Transaction(value = payload),
        factorSourceId = factorSourceId,
        ownedFactorInstances = ownedFactorInstances
    )

internal fun TransactionSignRequestInputOfSubintent.into()
    = TransactionSignRequestInput(
        payload = Signable.Payload.Subintent(value = payload),
        factorSourceId = factorSourceId,
        ownedFactorInstances = ownedFactorInstances
    )

internal fun TransactionSignRequestInputOfAuthIntent.into()
    = TransactionSignRequestInput(
        payload = Signable.Payload.Auth(value = payload),
        factorSourceId = factorSourceId,
        ownedFactorInstances = ownedFactorInstances
    )