package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

class ProfileTest: SampleTestable<Profile> {

    override val samples: List<Sample<Profile>>
        get() = listOf(Profile.sample)

    @Test
    fun testInit() {
        val hdFactorSource = PrivateHierarchicalDeterministicFactorSource.init(
            entropy = randomBagOfBytes(byteCount = 32),
            walletClientModel = WalletClientModel.ANDROID
        )

        val profile = Profile.init(
            privateHdFactorSource = hdFactorSource, creatingDeviceName = "Unit tests"
        )

        assertEquals("Unit tests - Android", profile.header.creatingDevice.description)
    }

}