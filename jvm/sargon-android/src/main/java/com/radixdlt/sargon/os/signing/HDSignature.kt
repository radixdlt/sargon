package com.radixdlt.sargon.os.signing

import com.radixdlt.sargon.HdSignatureOfAuthIntentHash
import com.radixdlt.sargon.HdSignatureOfSubintentHash
import com.radixdlt.sargon.HdSignatureOfTransactionIntentHash
import com.radixdlt.sargon.SignatureWithPublicKey

data class HdSignature<ID: Signable.ID> (
    /**
     * The input used to produce this `HDSignature`
     */
    val input: HdSignatureInput<ID>,
    /**
     * The ECDSA/EdDSA signature produced by the private key of the
     * `owned_hd_factor_instance.public_key`,
     * derived by the HDFactorSource identified by
     * `owned_hd_factor_
     * instance.factor_s
     * ource_id` and which
     * was derived at `owned_hd_factor_instance.derivation_path`.
     */
    val signature: SignatureWithPublicKey
)

fun HdSignature<Signable.ID.Transaction>.intoSargon() = HdSignatureOfTransactionIntentHash(
    input = input.intoSargon(),
    signature = signature
)

fun HdSignature<Signable.ID.Subintent>.intoSargon() = HdSignatureOfSubintentHash(
    input = input.intoSargon(),
    signature = signature
)

fun HdSignature<Signable.ID.Auth>.intoSargon() = HdSignatureOfAuthIntentHash(
    input = input.intoSargon(),
    signature = signature
)