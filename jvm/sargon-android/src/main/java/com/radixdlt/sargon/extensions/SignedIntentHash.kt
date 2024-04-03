package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.SignedIntentHash
import com.radixdlt.sargon.newSignedIntentHashFromString
import com.radixdlt.sargon.signedIntentHashFormatted

@Throws(SargonException::class)
fun SignedIntentHash.Companion.init(string: String) =
    newSignedIntentHashFromString(string = string)

fun SignedIntentHash.formatted(format: AddressFormat = AddressFormat.DEFAULT) =
    signedIntentHashFormatted(address = this, format = format)