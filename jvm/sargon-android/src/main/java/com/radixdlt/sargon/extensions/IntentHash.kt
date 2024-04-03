package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.IntentHash
import com.radixdlt.sargon.intentHashFormatted
import com.radixdlt.sargon.newIntentHashFromString

@Throws(SargonException::class)
fun IntentHash.Companion.init(string: String) =
    newIntentHashFromString(string = string)

fun IntentHash.formatted(format: AddressFormat = AddressFormat.DEFAULT) =
    intentHashFormatted(address = this, format = format)