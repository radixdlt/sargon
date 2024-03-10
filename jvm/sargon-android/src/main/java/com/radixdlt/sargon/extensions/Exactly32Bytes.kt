package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.newExactly32Bytes

fun Exactly32Bytes.Companion.init(bytes: BagOfBytes): Exactly32Bytes =
    newExactly32Bytes(bytes = bytes)