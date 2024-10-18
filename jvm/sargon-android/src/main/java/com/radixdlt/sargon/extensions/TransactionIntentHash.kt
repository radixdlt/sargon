package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.TransactionIntentHash
import com.radixdlt.sargon.transactionIntentHashFormatted
import com.radixdlt.sargon.newTransactionIntentHashFromString

@Throws(SargonException::class)
fun TransactionIntentHash.Companion.init(string: String) =
    newTransactionIntentHashFromString(string = string)

fun TransactionIntentHash.formatted(format: AddressFormat = AddressFormat.DEFAULT) =
    transactionIntentHashFormatted(address = this, format = format)