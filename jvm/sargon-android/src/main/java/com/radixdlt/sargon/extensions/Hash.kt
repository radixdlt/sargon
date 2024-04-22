package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.hashGetBytes

val Hash.bytes: Exactly32Bytes
    get() = hashGetBytes(hash = this)

val Hash.hex: String
    get() = bytes.hex