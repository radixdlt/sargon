package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.IdentifiedArray
import com.radixdlt.sargon.IdentifiedArrayImpl
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.ProfileNetwork

class ProfileNetworks private constructor(
    array: IdentifiedArray<NetworkId, ProfileNetwork>
) : IdentifiedArray<NetworkId, ProfileNetwork> by array {

    constructor(networks: List<ProfileNetwork>) : this(
        IdentifiedArrayImpl(
            elements = networks,
            identifier = { it.id }
        )
    )

    constructor(vararg network: ProfileNetwork) : this(networks = network.asList())
}