package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.SubintentHash
import com.radixdlt.sargon.newSubintentHashFromString
import com.radixdlt.sargon.subintentHashFormatted

@Throws(SargonException::class)
fun SubintentHash.Companion.init(string: String) =
    newSubintentHashFromString(string = string)

fun SubintentHash.formatted(format: AddressFormat = AddressFormat.DEFAULT) =
    subintentHashFormatted(address = this, format = format)