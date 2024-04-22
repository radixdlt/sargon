package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AuthorizedDapp
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.authorizedDappToJsonBytes
import com.radixdlt.sargon.newAuthorizedDappFromJsonBytes

@Throws(SargonException::class)
fun AuthorizedDapp.Companion.deserializeFromJsonBytes(jsonBytes: BagOfBytes) =
    newAuthorizedDappFromJsonBytes(jsonBytes = jsonBytes)

@Throws(SargonException::class)
fun AuthorizedDapp.Companion.deserializeFromJsonString(jsonString: String) =
    deserializeFromJsonBytes(jsonBytes = bagOfBytes(fromString = jsonString))

fun AuthorizedDapp.serializedJsonBytes(): BagOfBytes = authorizedDappToJsonBytes(authorizedDapp = this)
fun AuthorizedDapp.serializedJsonString(): String = serializedJsonBytes().string