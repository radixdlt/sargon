package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.RadixConnectMobileLinkRequest
import com.radixdlt.sargon.newRadixConnectMobileLinkRequestFromJsonBytes
import com.radixdlt.sargon.radixConnectMobileLinkRequestToJsonBytes

@Throws(SargonException::class)
fun RadixConnectMobileLinkRequest.Companion.fromJson(json: String) =
    newRadixConnectMobileLinkRequestFromJsonBytes(bagOfBytes(json))

fun RadixConnectMobileLinkRequest.toJson(): String = radixConnectMobileLinkRequestToJsonBytes(
    radixConnectMobileLinkRequest = this
).string