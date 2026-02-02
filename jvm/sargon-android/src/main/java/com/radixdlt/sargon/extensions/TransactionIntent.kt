package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.TransactionIntentHash
import com.radixdlt.sargon.CompiledTransactionIntent
import com.radixdlt.sargon.TransactionIntent
import com.radixdlt.sargon.transactionIntentCompile
import com.radixdlt.sargon.transactionIntentHash

fun TransactionIntent.hash(): TransactionIntentHash = transactionIntentHash(intent = this)

fun TransactionIntent.compile(): CompiledTransactionIntent = transactionIntentCompile(intent = this)