package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.hashGetBytes

val Hash.bytes: BagOfBytes
    get() = hashGetBytes(hash = this)

val Hash.hex: String
    get() = bytes.hex