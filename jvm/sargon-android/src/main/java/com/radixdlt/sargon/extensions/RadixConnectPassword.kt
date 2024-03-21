package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.RadixConnectPassword
import com.radixdlt.sargon.newRadixConnectPassword

fun RadixConnectPassword.Companion.init(bytes: Exactly32Bytes) =
    newRadixConnectPassword(bytes = bytes)