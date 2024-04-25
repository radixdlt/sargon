package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.hashGetBytes
import com.radixdlt.sargon.newHashFromBytes
import com.radixdlt.sargon.newHashFromString

@Throws(SargonException::class)
fun Hash.Companion.init(string: String) = newHashFromString(string = string)

fun Hash.Companion.init(bytes: Exactly32Bytes) = newHashFromBytes(bytes = bytes)

val Hash.bytes: Exactly32Bytes
    get() = hashGetBytes(hash = this)

val Hash.hex: String
    get() = bytes.hex