package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.newBagOfBytesFrom
import kotlin.random.Random

fun bagOfBytesOf(byteArray: ByteArray) = newBagOfBytesFrom(bytes = byteArray)

fun randomBagOfBytes(byteCount: Int) = Random.nextBytes(size = byteCount)