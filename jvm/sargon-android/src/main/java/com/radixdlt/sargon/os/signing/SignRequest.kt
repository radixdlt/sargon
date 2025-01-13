package com.radixdlt.sargon.os.signing

import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.SignRequestOfAuthIntent
import com.radixdlt.sargon.SignRequestOfSubintent
import com.radixdlt.sargon.SignRequestOfTransactionIntent

data class SignRequest<SP: Signable.Payload, ID: Signable.ID>(
    val factorSourceKind: FactorSourceKind,
    val perFactorSource: List<PerFactorSourceInput<SP, ID>>
) {
    companion object
}

fun SignRequestOfTransactionIntent.into() = SignRequest(
    factorSourceKind = factorSourceKind,
    perFactorSource = perFactorSource.map { it.into() }
)

fun SignRequestOfSubintent.into() = SignRequest(
    factorSourceKind = factorSourceKind,
    perFactorSource = perFactorSource.map { it.into() }
)

fun SignRequestOfAuthIntent.into() = SignRequest(
    factorSourceKind = factorSourceKind,
    perFactorSource = perFactorSource.map { it.into() }
)