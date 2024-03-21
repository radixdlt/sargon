package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Exactly29Bytes
import com.radixdlt.sargon.Exactly64Bytes
import com.radixdlt.sargon.exactly29BytesToBytes
import com.radixdlt.sargon.exactly29BytesToHex
import com.radixdlt.sargon.exactly64BytesToBytes
import com.radixdlt.sargon.exactly64BytesToHex
import com.radixdlt.sargon.newExactly64Bytes

@Throws(SargonException::class)
fun Exactly64Bytes.Companion.init(bytes: BagOfBytes): Exactly64Bytes =
    newExactly64Bytes(bytes = bytes)

val Exactly64Bytes.bytes: BagOfBytes
    get() = exactly64BytesToBytes(bytes = this)

val Exactly64Bytes.hex: String
    get() = exactly64BytesToHex(bytes = this)