package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.SignedTransactionIntentHash
import com.radixdlt.sargon.newSignedTransactionIntentHashFromString
import com.radixdlt.sargon.signedTransactionIntentHashFormatted

@Throws(SargonException::class)
fun SignedTransactionIntentHash.Companion.init(string: String) =
    newSignedTransactionIntentHashFromString(string = string)

fun SignedTransactionIntentHash.formatted(format: AddressFormat = AddressFormat.DEFAULT) =
    signedTransactionIntentHashFormatted(address = this, format = format)