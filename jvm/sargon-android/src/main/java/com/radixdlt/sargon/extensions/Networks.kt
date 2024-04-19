package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.ProfileNetwork
import com.radixdlt.sargon.ProfileNetworks
import com.radixdlt.sargon.newProfileNetworks
import com.radixdlt.sargon.newProfileNetworksByAppending
import com.radixdlt.sargon.newProfileNetworksRemovedById
import com.radixdlt.sargon.newProfileNetworksRemovedElement
import com.radixdlt.sargon.profileNetworksElementCount
import com.radixdlt.sargon.profileNetworksGetElements
import com.radixdlt.sargon.profileNetworksGetProfileNetworkById

fun ProfileNetworks.Companion.init(vararg network: ProfileNetwork): ProfileNetworks =
    init(networks = network.asList())

fun ProfileNetworks.Companion.init(networks: List<ProfileNetwork>): ProfileNetworks =
    newProfileNetworks(profileNetworks = networks)

operator fun ProfileNetworks.invoke() = profileNetworksGetElements(profileNetworks = this)

operator fun ProfileNetworks.get(index: Int) = invoke().get(index = index)

operator fun ProfileNetworks.contains(element: ProfileNetwork) = invoke().contains(element = element)

val ProfileNetworks.size: Int
    get() = profileNetworksElementCount(profileNetworks = this).toInt()

fun ProfileNetworks.append(network: ProfileNetwork): ProfileNetworks =
    newProfileNetworksByAppending(profileNetwork = network, to = this)

@Throws(SargonException::class)
fun ProfileNetworks.removeByNetworkId(networkId: NetworkId): ProfileNetworks =
    newProfileNetworksRemovedById(idOfProfileNetwork = networkId, from = this)

@Throws(SargonException::class)
fun ProfileNetworks.remove(network: ProfileNetwork): ProfileNetworks =
    newProfileNetworksRemovedElement(profileNetwork = network, from = this)

fun ProfileNetworks.getBy(networkId: NetworkId): ProfileNetwork? =
    profileNetworksGetProfileNetworkById(profileNetworks = this, id = networkId)