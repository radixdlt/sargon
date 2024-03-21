package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Exactly29Bytes
import com.radixdlt.sargon.Exactly33Bytes
import com.radixdlt.sargon.exactly29BytesToBytes
import com.radixdlt.sargon.exactly29BytesToHex
import com.radixdlt.sargon.exactly33BytesToBytes
import com.radixdlt.sargon.exactly33BytesToHex
import com.radixdlt.sargon.newExactly33Bytes

@Throws(SargonException::class)
fun Exactly33Bytes.Companion.init(bytes: BagOfBytes): Exactly33Bytes =
    newExactly33Bytes(bytes = bytes)

val Exactly33Bytes.bytes: BagOfBytes
    get() = exactly33BytesToBytes(bytes = this)

val Exactly33Bytes.hex: String
    get() = exactly33BytesToHex(bytes = this)