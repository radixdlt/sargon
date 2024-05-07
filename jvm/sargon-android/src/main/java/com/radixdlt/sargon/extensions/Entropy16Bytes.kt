package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Bip39Entropy
import com.radixdlt.sargon.Entropy16Bytes
import com.radixdlt.sargon.newEntropy16BytesFromBytes

fun Entropy16Bytes.Companion.random() = init(randomBagOfBytes(LENGTH))

@Throws(SargonException::class)
fun Entropy16Bytes.Companion.init(bytes: BagOfBytes): Entropy16Bytes =
    newEntropy16BytesFromBytes(bytes = bytes)

val Entropy16Bytes.Companion.LENGTH: Int
    get() = 16

fun Entropy16Bytes.asGeneral() = Bip39Entropy.EntropyOf16Bytes(this)