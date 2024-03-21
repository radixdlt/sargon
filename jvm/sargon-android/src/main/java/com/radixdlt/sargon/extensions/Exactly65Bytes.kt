package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Exactly29Bytes
import com.radixdlt.sargon.Exactly65Bytes
import com.radixdlt.sargon.exactly29BytesToBytes
import com.radixdlt.sargon.exactly29BytesToHex
import com.radixdlt.sargon.exactly65BytesToBytes
import com.radixdlt.sargon.exactly65BytesToHex
import com.radixdlt.sargon.newExactly65Bytes

@Throws(SargonException::class)
fun Exactly65Bytes.Companion.init(bytes: BagOfBytes): Exactly65Bytes =
    newExactly65Bytes(bytes = bytes)

val Exactly65Bytes.bytes: BagOfBytes
    get() = exactly65BytesToBytes(bytes = this)

val Exactly65Bytes.hex: String
    get() = exactly65BytesToHex(bytes = this)