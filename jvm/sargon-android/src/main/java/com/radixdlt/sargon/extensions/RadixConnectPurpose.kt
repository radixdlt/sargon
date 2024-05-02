package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.RadixConnectPurpose
import com.radixdlt.sargon.newRadixConnectPurposeFromString
import com.radixdlt.sargon.radixConnectPurposeToString

@Throws(SargonException::class)
fun RadixConnectPurpose.Companion.init(string: String) =
    newRadixConnectPurposeFromString(string = string)

val RadixConnectPurpose.string: String
    get() = radixConnectPurposeToString(kind = this)