package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Exactly29Bytes
import com.radixdlt.sargon.exactly29BytesToBytes
import com.radixdlt.sargon.exactly29BytesToHex
import com.radixdlt.sargon.newExactly29Bytes

@Throws(SargonException::class)
fun Exactly29Bytes.Companion.init(bytes: BagOfBytes): Exactly29Bytes =
    newExactly29Bytes(bytes = bytes)

val Exactly29Bytes.bytes: BagOfBytes
    get() = exactly29BytesToBytes(bytes = this)

val Exactly29Bytes.hex: String
    get() = exactly29BytesToHex(bytes = this)