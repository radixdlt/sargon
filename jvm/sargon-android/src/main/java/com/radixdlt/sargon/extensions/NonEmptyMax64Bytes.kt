package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.NonEmptyMax64Bytes
import com.radixdlt.sargon.newNonEmptyMax64Bytes

@Throws(SargonException::class)
fun NonEmptyMax64Bytes.Companion.init(bytes: BagOfBytes) =
    newNonEmptyMax64Bytes(bagOfBytes = bytes)