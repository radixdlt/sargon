package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Bip39Entropy
import com.radixdlt.sargon.Entropy32Bytes
import com.radixdlt.sargon.newEntropy32BytesFromBytes

fun Entropy32Bytes.Companion.random() = init(randomBagOfBytes(LENGTH))

@Throws(SargonException::class)
fun Entropy32Bytes.Companion.init(bytes: BagOfBytes): Entropy32Bytes =
    newEntropy32BytesFromBytes(bytes = bytes)

val Entropy32Bytes.Companion.LENGTH: Int
    get() = 32

fun Entropy32Bytes.asGeneral() = Bip39Entropy.EntropyOf32Bytes(this)