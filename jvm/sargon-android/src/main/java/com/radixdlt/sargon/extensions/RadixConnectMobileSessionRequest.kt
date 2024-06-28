package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.RadixConnectMobileSessionRequest
import com.radixdlt.sargon.newRadixConnectMobileSessionRequestFromJsonBytes
import com.radixdlt.sargon.radixConnectMobileSessionRequestToJsonBytes

@Throws(SargonException::class)
fun RadixConnectMobileSessionRequest.Companion.fromJson(json: String) =
    newRadixConnectMobileSessionRequestFromJsonBytes(bagOfBytes(json))

fun RadixConnectMobileSessionRequest.toJson(): String = radixConnectMobileSessionRequestToJsonBytes(
    radixConnectMobileSessionRequest = this
).string