package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Bip39Entropy
import com.radixdlt.sargon.Entropy20Bytes
import com.radixdlt.sargon.Entropy24Bytes
import com.radixdlt.sargon.newEntropy24BytesFromBytes

fun Entropy24Bytes.Companion.random() = init(randomBagOfBytes(LENGTH))

@Throws(SargonException::class)
fun Entropy24Bytes.Companion.init(bytes: BagOfBytes): Entropy24Bytes =
    newEntropy24BytesFromBytes(bytes = bytes)

val Entropy24Bytes.Companion.LENGTH: Int
    get() = 24

fun Entropy24Bytes.asGeneral() = Bip39Entropy.EntropyOf24Bytes(this)