package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Exactly29Bytes
import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.exactly29BytesToBytes
import com.radixdlt.sargon.exactly29BytesToHex
import com.radixdlt.sargon.exactly32BytesToBytes
import com.radixdlt.sargon.exactly32BytesToHex
import com.radixdlt.sargon.newExactly32Bytes

@Throws(SargonException::class)
fun Exactly32Bytes.Companion.init(bytes: BagOfBytes): Exactly32Bytes =
    newExactly32Bytes(bytes = bytes)

val Exactly32Bytes.bytes: BagOfBytes
    get() = exactly32BytesToBytes(bytes = this)

val Exactly32Bytes.hex: String
    get() = exactly32BytesToHex(bytes = this)