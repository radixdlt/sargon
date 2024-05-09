package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.RadixConnectPurpose
import com.radixdlt.sargon.newRadixConnectPurposeFromString

@Throws(SargonException::class)
fun RadixConnectPurpose.Companion.init(string: String) =
    newRadixConnectPurposeFromString(string = string)