package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AuthIntent
import com.radixdlt.sargon.AuthIntentHash
import com.radixdlt.sargon.authIntentGetHash

fun AuthIntent.hash(): AuthIntentHash = authIntentGetHash(authIntent = this)