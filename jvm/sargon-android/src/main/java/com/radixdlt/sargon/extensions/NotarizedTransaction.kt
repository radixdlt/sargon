package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NotarizedTransaction
import com.radixdlt.sargon.notarizedTransactionCompile

fun NotarizedTransaction.compile() = notarizedTransactionCompile(notarizedTransaction = this)