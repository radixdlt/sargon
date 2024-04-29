package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.MobileConnectRequest
import com.radixdlt.sargon.newMobileConnectRequest

@Throws(SargonException::class)
fun MobileConnectRequest.Companion.parseFrom(url: String) = runCatching {
    newMobileConnectRequest(url = url)
}
