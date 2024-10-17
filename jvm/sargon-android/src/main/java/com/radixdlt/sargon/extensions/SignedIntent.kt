package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.SignedIntent
import com.radixdlt.sargon.SignedTransactionIntentHash
import com.radixdlt.sargon.signedIntentHash

fun SignedIntent.hash(): SignedTransactionIntentHash = signedIntentHash(signedIntent = this)