package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.NonEmptyMax64Bytes
import com.radixdlt.sargon.newNonEmptyMax64Bytes

fun NonEmptyMax64Bytes.Companion.init(bagOfBytes: BagOfBytes) =
    newNonEmptyMax64Bytes(bagOfBytes = bagOfBytes)