package com.radixdlt.sargon.os.signing

import com.radixdlt.sargon.AddressOfAccountOrPersona
import com.radixdlt.sargon.InvalidTransactionIfNeglectedOfAuthIntentHash
import com.radixdlt.sargon.InvalidTransactionIfNeglectedOfSubintentHash
import com.radixdlt.sargon.InvalidTransactionIfNeglectedOfTransactionIntentHash

/**
 * A list of entities which would fail in a transaction if we would
 * neglect certain factor source, either by user explicitly skipping
 * it or if implicitly neglected due to failure.
 */
data class InvalidTransactionIfNeglected<ID: Signable.ID>(
    /**
     * The intent hash of the transaction which would be invalid if a
     * certain factor source would be neglected, either if user
     * explicitly skipped it or implicitly neglected due to failure.
     */
    val signableId: ID,
    /**
     * The entities in the transaction which would fail auth.
     */
    val entitiesWhichWouldFailAuth: List<AddressOfAccountOrPersona>
)

internal fun InvalidTransactionIfNeglectedOfTransactionIntentHash.into()
    = InvalidTransactionIfNeglected(
        signableId = Signable.ID.Transaction(value = signableId),
        entitiesWhichWouldFailAuth = entitiesWhichWouldFailAuth
    )

internal fun InvalidTransactionIfNeglectedOfSubintentHash.into()
    = InvalidTransactionIfNeglected(
        signableId = Signable.ID.Subintent(value = signableId),
        entitiesWhichWouldFailAuth = entitiesWhichWouldFailAuth
    )

internal fun InvalidTransactionIfNeglectedOfAuthIntentHash.into()
    = InvalidTransactionIfNeglected(
        signableId = Signable.ID.Auth(value = signableId),
        entitiesWhichWouldFailAuth = entitiesWhichWouldFailAuth
    )