package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.ProfileNetworks
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet

internal class ProfileNetworksTest: IdentifiedArrayTest<ProfileNetworks, NetworkId, ProfileNetwork>()  {
    override fun element(): ProfileNetwork = ProfileNetwork.sampleMainnet()

    override fun elementWithDifferentId(): ProfileNetwork = ProfileNetwork.sampleStokenet()

    override fun identifier(element: ProfileNetwork): NetworkId = element.id

    override fun init(element: ProfileNetwork): ProfileNetworks = ProfileNetworks(element)
}