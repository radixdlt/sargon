package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Nonce
import com.radixdlt.sargon.newNonceRandom
import com.radixdlt.sargon.nonceGetValue

fun Nonce.Companion.random(): Nonce = newNonceRandom()

val Nonce.value: UInt
    get() = nonceGetValue(nonce = this)