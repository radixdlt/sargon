package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.RadixConnectMobileConnectRequest
import com.radixdlt.sargon.newMobileConnectRequest

fun RadixConnectMobileConnectRequest.Companion.parseFrom(url: String) = runCatching {
    newMobileConnectRequest(url = url)
}
