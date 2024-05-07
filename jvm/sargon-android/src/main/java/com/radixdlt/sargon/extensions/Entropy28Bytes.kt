package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Bip39Entropy
import com.radixdlt.sargon.Entropy24Bytes
import com.radixdlt.sargon.Entropy28Bytes
import com.radixdlt.sargon.newEntropy28BytesFromBytes

fun Entropy28Bytes.Companion.random() = init(randomBagOfBytes(LENGTH))

@Throws(SargonException::class)
fun Entropy28Bytes.Companion.init(bytes: BagOfBytes): Entropy28Bytes =
    newEntropy28BytesFromBytes(bytes = bytes)

val Entropy28Bytes.Companion.LENGTH: Int
    get() = 28

fun Entropy28Bytes.asGeneral() = Bip39Entropy.EntropyOf28Bytes(this)