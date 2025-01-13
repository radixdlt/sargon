package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.PerFactorOutcomeOfAuthIntentHash
import com.radixdlt.sargon.PerFactorOutcomeOfSubintentHash
import com.radixdlt.sargon.PerFactorOutcomeOfTransactionIntentHash
import com.radixdlt.sargon.SignResponseOfAuthIntentHash
import com.radixdlt.sargon.SignResponseOfSubintentHash
import com.radixdlt.sargon.SignResponseOfTransactionIntentHash
import com.radixdlt.sargon.newSignResponseOfAuthIntentHashFromOutcomes
import com.radixdlt.sargon.newSignResponseOfSubintentHashFromOutcomes
import com.radixdlt.sargon.newSignResponseOfTransactionIntentHashFromOutcomes

@Throws(CommonException::class)
fun SignResponseOfTransactionIntentHash.Companion.fromOutcomes(
    outcomes: List<PerFactorOutcomeOfTransactionIntentHash>
) = newSignResponseOfTransactionIntentHashFromOutcomes(outcomes = outcomes)

@Throws(CommonException::class)
fun SignResponseOfSubintentHash.Companion.fromOutcomes(
    outcomes: List<PerFactorOutcomeOfSubintentHash>
) = newSignResponseOfSubintentHashFromOutcomes(outcomes = outcomes)

@Throws(CommonException::class)
fun SignResponseOfAuthIntentHash.Companion.fromOutcomes(
    outcomes: List<PerFactorOutcomeOfAuthIntentHash>
) = newSignResponseOfAuthIntentHashFromOutcomes(outcomes = outcomes)