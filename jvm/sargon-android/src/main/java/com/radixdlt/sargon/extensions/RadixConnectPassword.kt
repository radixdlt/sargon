package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.RadixConnectPassword
import com.radixdlt.sargon.newRadixConnectPassword
import com.radixdlt.sargon.radixConnectPasswordMessageHash

fun RadixConnectPassword.Companion.init(bytes: Exactly32Bytes) =
    newRadixConnectPassword(bytes = bytes)

fun RadixConnectPassword.messageHash(): Hash =
    radixConnectPasswordMessageHash(this)