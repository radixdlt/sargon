package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.CompiledTransactionIntent
import com.radixdlt.sargon.TransactionIntent
import com.radixdlt.sargon.compiledTransactionIntentDecompile

fun CompiledTransactionIntent.decompile(): TransactionIntent =
    compiledTransactionIntentDecompile(compiledIntent = this)