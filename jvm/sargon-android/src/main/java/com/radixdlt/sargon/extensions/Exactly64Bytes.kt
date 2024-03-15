package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Exactly64Bytes
import com.radixdlt.sargon.newExactly64Bytes

fun Exactly64Bytes.Companion.init(bytes: BagOfBytes): Exactly64Bytes =
    newExactly64Bytes(bytes = bytes)