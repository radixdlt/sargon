@file:OptIn(ExperimentalUnsignedTypes::class, ExperimentalStdlibApi::class)

package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.newBagOfBytesFrom
import kotlin.random.Random

fun bagOfBytesOf(byteArray: ByteArray) = newBagOfBytesFrom(bytes = byteArray)

fun randomBagOfBytes(byteCount: Int) = Random.nextBytes(size = byteCount).toBagOfBytes()

fun String.hexToBagOfBytes(): BagOfBytes {
    check(length % 2 == 0) { "Must have an even length" }

    return chunked(2).map { it.toInt(16).toByte() }.toByteArray().toBagOfBytes()
}

fun ByteArray.toBagOfBytes() = newBagOfBytesFrom(bytes = this)

val BagOfBytes.hex: String
    get() = toUByteArray().toHexString()