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
fun Profile.Companion.deserializeFromBytes(jsonBytes: BagOfBytes)
    = newProfileFromJsonBytes(jsonBytes = jsonBytes)

@Throws(SargonException::class)
fun Profile.Companion.deserializeFromString(jsonString: String)
        = deserializeFromBytes(jsonBytes = bagOfBytes(fromString = jsonString))

fun Profile.serializedBytes() = profileToJsonBytes(profile = this)

fun Profile.serializedString() = serializedBytes().string