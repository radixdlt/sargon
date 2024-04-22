@file:OptIn(ExperimentalUnsignedTypes::class, ExperimentalStdlibApi::class)

package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.hash
import com.radixdlt.sargon.newBagOfBytesFrom
import kotlin.random.Random

internal fun bagOfBytes(fromString: String) = fromString
    .toByteArray(charset = Charsets.UTF_8)
    .toBagOfBytes()

internal val BagOfBytes.string: String
    get() = toUByteArray().toByteArray().toString(charset = Charsets.UTF_8)

fun bagOfBytesOf(byteArray: ByteArray) = newBagOfBytesFrom(bytes = byteArray)

fun String.hexToBagOfBytes(): BagOfBytes {
    check(length % 2 == 0) { "Must have an even length" }

    return chunked(2).map { it.toInt(16).toByte() }.toByteArray().toBagOfBytes()
}

fun ByteArray.toBagOfBytes() = bagOfBytesOf(byteArray = this)

val BagOfBytes.hex: String
    get() = toUByteArray().toHexString()

fun BagOfBytes.hash(): Hash = hash(data = this)

internal fun randomBagOfBytes(byteCount: Int) = Random.nextBytes(size = byteCount).toBagOfBytes()