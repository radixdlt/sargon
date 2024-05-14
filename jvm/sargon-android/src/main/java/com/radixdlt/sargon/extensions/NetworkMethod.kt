package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NetworkMethod
import com.radixdlt.sargon.networkMethodToString

fun NetworkMethod.toHttpMethod(): String = networkMethodToString(method = this)