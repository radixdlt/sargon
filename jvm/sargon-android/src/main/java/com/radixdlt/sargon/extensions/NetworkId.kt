package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.networkIdDiscriminant
import com.radixdlt.sargon.networkIdToString
import com.radixdlt.sargon.newNetworkIdFromDiscriminant

@Throws(SargonException::class)
fun NetworkId.Companion.init(discriminant: UByte) =
    newNetworkIdFromDiscriminant(discriminant = discriminant)

val NetworkId.string: String
    get() = networkIdToString(id = this)

val NetworkId.discriminant: UByte
    get() = networkIdDiscriminant(id = this)