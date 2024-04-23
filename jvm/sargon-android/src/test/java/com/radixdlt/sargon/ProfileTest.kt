package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.bagOfBytes
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.deserializeFromJsonString
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.serializedJsonString
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ProfileTest: SampleTestable<Profile> {

    override val samples: List<Sample<Profile>>
        get() = listOf(Profile.sample)

    @Test
    fun testInit() {
        val hdFactorSource = PrivateHierarchicalDeterministicFactorSource.init(
            isMainBDFS = true,
            entropy = NonEmptyMax32Bytes(bagOfBytes = randomBagOfBytes(byteCount = 32)),
            walletClientModel = WalletClientModel.ANDROID
        )

        val profile = Profile.init(
            deviceFactorSource = hdFactorSource.factorSource,
            creatingDeviceName = "Unit tests"
        )

        assertEquals("Unit tests - Android", profile.header.creatingDevice.description)
    }

    @Test
    fun testRoundtrip() {
        val sut = Profile.sample()

        assertEquals(sut, Profile.deserializeFromJsonString(jsonString = sut.serializedJsonString()))
    }

    @Test
    fun testInitFromMalformedJson() {
        val json = "{}"

        val result = runCatching { Profile.deserializeFromJsonString(jsonString = json) }.exceptionOrNull()
                as? CommonException.FailedToDeserializeJsonToValue

        assertEquals(
            bagOfBytes(json).size.toULong(),
            result?.jsonByteCount
        )
        assertEquals(
            "Profile",
            result?.typeName
        )
    }
}