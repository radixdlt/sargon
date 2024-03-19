package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Exactly33Bytes
import com.radixdlt.sargon.newExactly33Bytes

@Throws(SargonException::class)
fun Exactly33Bytes.Companion.init(bytes: BagOfBytes): Exactly33Bytes =
    newExactly33Bytes(bytes = bytes)