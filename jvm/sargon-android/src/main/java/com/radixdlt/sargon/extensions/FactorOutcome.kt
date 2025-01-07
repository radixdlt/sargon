package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.FactorOutcomeOfAuthIntentHash
import com.radixdlt.sargon.FactorOutcomeOfSubintentHash
import com.radixdlt.sargon.FactorOutcomeOfTransactionIntentHash
import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.HdSignatureOfAuthIntentHash
import com.radixdlt.sargon.HdSignatureOfSubintentHash
import com.radixdlt.sargon.HdSignatureOfTransactionIntentHash
import com.radixdlt.sargon.newFailureFactorOutcomeOfAuthIntentHash
import com.radixdlt.sargon.newFailureFactorOutcomeOfSubintentHash
import com.radixdlt.sargon.newFailureFactorOutcomeOfTransactionIntentHash
import com.radixdlt.sargon.newSignedFactorOutcomeOfAuthIntentHash
import com.radixdlt.sargon.newSignedFactorOutcomeOfSubintentHash
import com.radixdlt.sargon.newSignedFactorOutcomeOfTransactionIntentHash
import com.radixdlt.sargon.newSkippedFactorOutcomeOfAuthIntentHash
import com.radixdlt.sargon.newSkippedFactorOutcomeOfSubintentHash
import com.radixdlt.sargon.newSkippedFactorOutcomeOfTransactionIntentHash

@Throws(CommonException::class)
fun FactorOutcomeOfTransactionIntentHash.Companion.signed(
    producedSignatures: List<HdSignatureOfTransactionIntentHash>
): FactorOutcomeOfTransactionIntentHash = newSignedFactorOutcomeOfTransactionIntentHash(
    producedSignatures = producedSignatures
)

@Throws(CommonException::class)
fun FactorOutcomeOfSubintentHash.Companion.signed(
    producedSignatures: List<HdSignatureOfSubintentHash>
): FactorOutcomeOfSubintentHash = newSignedFactorOutcomeOfSubintentHash(
    producedSignatures = producedSignatures
)

@Throws(CommonException::class)
fun FactorOutcomeOfAuthIntentHash.Companion.signed(
    producedSignatures: List<HdSignatureOfAuthIntentHash>
): FactorOutcomeOfAuthIntentHash = newSignedFactorOutcomeOfAuthIntentHash(
    producedSignatures = producedSignatures
)

fun FactorOutcomeOfTransactionIntentHash.Companion.skipped(
    factorSourceId: FactorSourceIdFromHash
): FactorOutcomeOfTransactionIntentHash = newSkippedFactorOutcomeOfTransactionIntentHash(
    factorSourceId = factorSourceId
)

fun FactorOutcomeOfSubintentHash.Companion.skipped(
    factorSourceId: FactorSourceIdFromHash
): FactorOutcomeOfSubintentHash = newSkippedFactorOutcomeOfSubintentHash(
    factorSourceId = factorSourceId
)

fun FactorOutcomeOfAuthIntentHash.Companion.skipped(
    factorSourceId: FactorSourceIdFromHash
): FactorOutcomeOfAuthIntentHash = newSkippedFactorOutcomeOfAuthIntentHash(
    factorSourceId = factorSourceId
)

fun FactorOutcomeOfTransactionIntentHash.Companion.failure(
    factorSourceId: FactorSourceIdFromHash
): FactorOutcomeOfTransactionIntentHash = newFailureFactorOutcomeOfTransactionIntentHash(
    factorSourceId = factorSourceId
)

fun FactorOutcomeOfSubintentHash.Companion.failure(
    factorSourceId: FactorSourceIdFromHash
): FactorOutcomeOfSubintentHash = newFailureFactorOutcomeOfSubintentHash(
    factorSourceId = factorSourceId
)

fun FactorOutcomeOfAuthIntentHash.Companion.failure(
    factorSourceId: FactorSourceIdFromHash
): FactorOutcomeOfAuthIntentHash = newFailureFactorOutcomeOfAuthIntentHash(
    factorSourceId = factorSourceId
)

