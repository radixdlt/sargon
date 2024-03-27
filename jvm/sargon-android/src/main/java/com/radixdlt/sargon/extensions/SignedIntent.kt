package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.SignedIntent
import com.radixdlt.sargon.SignedIntentHash
import com.radixdlt.sargon.signedIntentHash

fun SignedIntent.hash(): SignedIntentHash = signedIntentHash(signedIntent = this)