package com.radixdlt.sargon.os.signing

import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.FactorOutcomeOfAuthIntentHash
import com.radixdlt.sargon.FactorOutcomeOfSubintentHash
import com.radixdlt.sargon.FactorOutcomeOfTransactionIntentHash
import com.radixdlt.sargon.NeglectedFactor
import com.radixdlt.sargon.extensions.signed
import kotlin.jvm.Throws

sealed interface FactorOutcome<ID : Signable.ID> {

    /**
     * The user successfully signed with the factor source, the associated
     * value contains the produced signatures and any relevant metadata.
     */
    data class Signed<ID : Signable.ID>(
        val producedSignatures: List<HdSignature<ID>>
    ) : FactorOutcome<ID>

    /**
     * The factor source got neglected, either due to user explicitly skipping
     * or due to failure
     */
    data class Neglected<ID : Signable.ID>(
        val factor: NeglectedFactor
    ) : FactorOutcome<ID>
}

@Throws(CommonException::class)
fun FactorOutcome<Signable.ID.Transaction>.intoSargon() = when (this) {
    is FactorOutcome.Signed -> intoSargon()
    is FactorOutcome.Neglected -> intoSargon()
}

@Throws(CommonException::class)
fun FactorOutcome<Signable.ID.Subintent>.intoSargon() = when (this) {
    is FactorOutcome.Signed -> intoSargon()
    is FactorOutcome.Neglected -> intoSargon()
}

@Throws(CommonException::class)
fun FactorOutcome<Signable.ID.Auth>.intoSargon() = when (this) {
    is FactorOutcome.Signed -> intoSargon()
    is FactorOutcome.Neglected -> intoSargon()
}


@Throws(CommonException::class)
fun FactorOutcome.Signed<Signable.ID.Transaction>.intoSargon() =
    FactorOutcomeOfTransactionIntentHash.signed(
        producedSignatures = producedSignatures.map { it.intoSargon() }
    )

@Throws(CommonException::class)
fun FactorOutcome.Signed<Signable.ID.Subintent>.intoSargon() =
    FactorOutcomeOfSubintentHash.signed(
        producedSignatures = producedSignatures.map { it.intoSargon() }
    )


fun FactorOutcome.Signed<Signable.ID.Auth>.intoSargon() =
    FactorOutcomeOfAuthIntentHash.signed(
        producedSignatures = producedSignatures.map { it.intoSargon() }
    )

fun FactorOutcome.Neglected<Signable.ID.Transaction>.intoSargon() =
    FactorOutcomeOfTransactionIntentHash.Neglected(v1 = factor)

fun FactorOutcome.Neglected<Signable.ID.Subintent>.intoSargon() =
    FactorOutcomeOfSubintentHash.Neglected(v1 = factor)

fun FactorOutcome.Neglected<Signable.ID.Auth>.intoSargon() =
    FactorOutcomeOfAuthIntentHash.Neglected(v1 = factor)