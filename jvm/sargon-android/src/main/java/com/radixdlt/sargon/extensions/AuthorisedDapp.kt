package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AuthorizedDapp
import com.radixdlt.sargon.authorizedDappToJsonBytes
import com.radixdlt.sargon.newAuthorizedDappFromJsonBytes

@Throws(SargonException::class)
fun AuthorizedDapp.Companion.fromJson(jsonString: String) =
    newAuthorizedDappFromJsonBytes(jsonBytes = bagOfBytes(fromString = jsonString))

fun AuthorizedDapp.toJson(): String = authorizedDappToJsonBytes(authorizedDapp = this).string