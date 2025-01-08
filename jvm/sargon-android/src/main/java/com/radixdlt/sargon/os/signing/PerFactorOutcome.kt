package com.radixdlt.sargon.os.signing

import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.PerFactorOutcomeOfAuthIntentHash
import com.radixdlt.sargon.PerFactorOutcomeOfSubintentHash
import com.radixdlt.sargon.PerFactorOutcomeOfTransactionIntentHash
import kotlin.jvm.Throws

data class PerFactorOutcome<ID : Signable.ID>(
    val factorSourceId: FactorSourceIdFromHash,
    val outcome: FactorOutcome<ID>
)

@Throws(CommonException::class)
fun PerFactorOutcome<Signable.ID.Transaction>.intoSargon() =
    PerFactorOutcomeOfTransactionIntentHash(
        factorSourceId = factorSourceId,
        outcome = outcome.intoSargon()
    )

@Throws(CommonException::class)
fun PerFactorOutcome<Signable.ID.Subintent>.intoSargon() = PerFactorOutcomeOfSubintentHash(
    factorSourceId = factorSourceId,
    outcome = outcome.intoSargon()
)

@Throws(CommonException::class)
fun PerFactorOutcome<Signable.ID.Auth>.intoSargon() = PerFactorOutcomeOfAuthIntentHash(
    factorSourceId = factorSourceId,
    outcome = outcome.intoSargon()
)