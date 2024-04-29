package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NetworkMethod

fun NetworkMethod.toHttpMethod(): String = when (this) {
    NetworkMethod.POST -> "POST"
    NetworkMethod.GET -> "GET"
}