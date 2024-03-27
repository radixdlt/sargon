package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.IntentHash
import com.radixdlt.sargon.TransactionIntent
import com.radixdlt.sargon.transactionIntentCompile
import com.radixdlt.sargon.transactionIntentHash

fun TransactionIntent.hash(): IntentHash = transactionIntentHash(intent = this)

fun TransactionIntent.compile(): BagOfBytes = transactionIntentCompile(intent = this)