package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.SignedIntentHash
import com.radixdlt.sargon.newSignedIntentHashFromString

@Throws(SargonException::class)
fun SignedIntentHash.Companion.init(string: String) =
    newSignedIntentHashFromString(string = string)