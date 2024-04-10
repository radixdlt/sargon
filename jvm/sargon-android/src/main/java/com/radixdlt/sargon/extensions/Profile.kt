package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.PrivateHierarchicalDeterministicFactorSource
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.newProfile
import com.radixdlt.sargon.newProfileFromJsonBytes
import com.radixdlt.sargon.profileToJsonBytes

fun Profile.Companion.init(
    privateHdFactorSource: PrivateHierarchicalDeterministicFactorSource,
    creatingDeviceName: String
) = newProfile(
    privateHdFactorSource = privateHdFactorSource,
    creatingDeviceName = creatingDeviceName
)

@Throws(SargonException::class)
fun Profile.Companion.init(json: BagOfBytes) = newProfileFromJsonBytes(json = json)

@Throws(SargonException::class)
fun Profile.snapshotJson() = profileToJsonBytes(profile = this)