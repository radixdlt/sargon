package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Exactly29Bytes
import com.radixdlt.sargon.newExactly29Bytes

fun Exactly29Bytes.Companion.init(bytes: BagOfBytes): Exactly29Bytes =
    newExactly29Bytes(bytes = bytes)