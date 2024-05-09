package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.ProfileNetwork
import com.radixdlt.sargon.annotation.KoverIgnore

class ProfileNetworks private constructor(
    private val array: IdentifiedArray<NetworkId, ProfileNetwork>
) : IdentifiedArray<NetworkId, ProfileNetwork> by array {

    constructor(networks: List<ProfileNetwork>) : this(
        IdentifiedArrayImpl(
            elements = networks,
            identifier = { it.id }
        )
    )

    constructor(vararg network: ProfileNetwork) : this(networks = network.asList())

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as ProfileNetworks

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "ProfileNetworks(array=$array)"
    }

}

fun List<ProfileNetwork>.asIdentifiable() = ProfileNetworks(networks = this)
