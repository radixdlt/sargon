package com.radixdlt.sargon.os.signing

import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.SignResponseOfAuthIntentHash
import com.radixdlt.sargon.SignResponseOfSubintentHash
import com.radixdlt.sargon.SignResponseOfTransactionIntentHash
import com.radixdlt.sargon.extensions.fromOutcomes
import kotlin.jvm.Throws

data class SignResponse<ID: Signable.ID>(
    val perFactorOutcome: List<PerFactorOutcome<ID>>
)

@Throws(CommonException::class)
fun SignResponse<Signable.ID.Transaction>.intoSargon()
    = SignResponseOfTransactionIntentHash.fromOutcomes(
        outcomes = perFactorOutcome.map { it.intoSargon() }
    )

@Throws(CommonException::class)
fun SignResponse<Signable.ID.Subintent>.intoSargon()
    = SignResponseOfSubintentHash.fromOutcomes(
        outcomes = perFactorOutcome.map { it.intoSargon() }
    )

@Throws(CommonException::class)
fun SignResponse<Signable.ID.Auth>.intoSargon()
    = SignResponseOfAuthIntentHash.fromOutcomes(
        outcomes = perFactorOutcome.map { it.intoSargon() }
    )