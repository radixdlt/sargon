package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.P2pTransportProfile
import com.radixdlt.sargon.SavedP2pTransportProfiles
import com.radixdlt.sargon.newSavedP2pTransportProfiles
import com.radixdlt.sargon.newSavedP2pTransportProfilesDefault
import com.radixdlt.sargon.savedP2pTransportProfilesGetAllElements

fun SavedP2pTransportProfiles.Companion.init(current: P2pTransportProfile) =
    newSavedP2pTransportProfiles(current = current)

val SavedP2pTransportProfiles.Companion.default: SavedP2pTransportProfiles
    get() = newSavedP2pTransportProfilesDefault()

val SavedP2pTransportProfiles.all: List<P2pTransportProfile>
    get() = savedP2pTransportProfilesGetAllElements(profiles = this)

fun SavedP2pTransportProfiles.append(profile: P2pTransportProfile): Boolean {
    if (current.signalingServer == profile.signalingServer || other.any { it.signalingServer == profile.signalingServer }) {
        return false
    }
    other = other + profile
    return true
}

fun SavedP2pTransportProfiles.remove(profile: P2pTransportProfile): Boolean {
    val oldCount = other.size
    other = other.filterNot { it.signalingServer == profile.signalingServer }
    return oldCount != other.size
}

fun SavedP2pTransportProfiles.changeCurrent(newCurrent: P2pTransportProfile): Boolean {
    if (current.signalingServer == newCurrent.signalingServer) {
        return false
    }

    val oldCurrent = current
    other = other.filterNot { it.signalingServer == newCurrent.signalingServer }
    current = newCurrent

    if (other.none { it.signalingServer == oldCurrent.signalingServer }) {
        other = other + oldCurrent
    }
    return true
}
