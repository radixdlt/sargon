package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.PublicKeyHash
import com.radixdlt.sargon.newPublicKeyHashOfKey

fun PublicKeyHash.Companion.init(publicKey: PublicKey): PublicKeyHash =
    newPublicKeyHashOfKey(publicKey = publicKey)