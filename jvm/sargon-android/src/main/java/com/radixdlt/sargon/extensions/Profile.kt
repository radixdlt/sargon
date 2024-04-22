package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.DeviceFactorSource
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.newProfile
import com.radixdlt.sargon.newProfileFromJsonBytes
import com.radixdlt.sargon.profileToJsonBytes

fun Profile.Companion.init(
    deviceFactorSource: DeviceFactorSource,
    creatingDeviceName: String
) = newProfile(
    deviceFactorSource = deviceFactorSource,
    creatingDeviceName = creatingDeviceName
)

@Throws(SargonException::class)
fun Profile.Companion.deserializeFromJsonBytes(jsonBytes: BagOfBytes)
    = newProfileFromJsonBytes(jsonBytes = jsonBytes)

@Throws(SargonException::class)
fun Profile.Companion.deserializeFromJsonString(jsonString: String)
        = deserializeFromJsonBytes(jsonBytes = bagOfBytes(fromString = jsonString))

fun Profile.serializedJsonBytes() = profileToJsonBytes(profile = this)

fun Profile.serializedJsonString() = serializedJsonBytes().string