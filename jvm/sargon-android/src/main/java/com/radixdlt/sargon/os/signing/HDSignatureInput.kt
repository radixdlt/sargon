package com.radixdlt.sargon.os.signing

import com.radixdlt.sargon.HdSignatureInputOfAuthIntentHash
import com.radixdlt.sargon.HdSignatureInputOfSubintentHash
import com.radixdlt.sargon.HdSignatureInputOfTransactionIntentHash
import com.radixdlt.sargon.HdSignatureOfTransactionIntentHash
import com.radixdlt.sargon.OwnedFactorInstance

data class HdSignatureInput<ID: Signable.ID> (
    /**
     * Hash which was signed.
     */
    val payloadId: ID,
    /**
     * The account or identity address of the entity which signed the hash,
     * with expected public key and with derivation path to derive PrivateKey
     * with.
     */
    val ownedFactorInstance: OwnedFactorInstance
)

fun HdSignatureInput<Signable.ID.Transaction>.intoSargon()
    = HdSignatureInputOfTransactionIntentHash(
        payloadId = payloadId.value,
        ownedFactorInstance = ownedFactorInstance
    )

fun HdSignatureInput<Signable.ID.Subintent>.intoSargon()
        = HdSignatureInputOfSubintentHash(
    payloadId = payloadId.value,
    ownedFactorInstance = ownedFactorInstance
)

fun HdSignatureInput<Signable.ID.Auth>.intoSargon()
        = HdSignatureInputOfAuthIntentHash(
    payloadId = payloadId.value,
    ownedFactorInstance = ownedFactorInstance
)