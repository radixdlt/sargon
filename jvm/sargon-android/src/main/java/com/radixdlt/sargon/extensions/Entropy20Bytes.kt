package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Bip39Entropy
import com.radixdlt.sargon.Entropy16Bytes
import com.radixdlt.sargon.Entropy20Bytes
import com.radixdlt.sargon.newEntropy20BytesFromBytes

fun Entropy20Bytes.Companion.random() = init(randomBagOfBytes(LENGTH))

@Throws(SargonException::class)
fun Entropy20Bytes.Companion.init(bytes: BagOfBytes): Entropy20Bytes =
    newEntropy20BytesFromBytes(bytes = bytes)

val Entropy20Bytes.Companion.LENGTH: Int
    get() = 20

fun Entropy20Bytes.asGeneral() = Bip39Entropy.EntropyOf20Bytes(this)