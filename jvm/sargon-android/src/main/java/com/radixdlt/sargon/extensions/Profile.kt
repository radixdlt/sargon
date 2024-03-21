package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.PrivateHierarchicalDeterministicFactorSource
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.newProfile

fun Profile.Companion.init(
    privateHdFactorSource: PrivateHierarchicalDeterministicFactorSource,
    creatingDeviceName: String
) = newProfile(
    privateHdFactorSource = privateHdFactorSource,
    creatingDeviceName = creatingDeviceName
)