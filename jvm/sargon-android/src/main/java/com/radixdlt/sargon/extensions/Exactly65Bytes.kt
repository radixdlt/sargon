package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Exactly65Bytes
import com.radixdlt.sargon.newExactly65Bytes

fun Exactly65Bytes.Companion.init(bytes: BagOfBytes): Exactly65Bytes =
    newExactly65Bytes(bytes = bytes)