package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.analyzeContentsOfFile
import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.bagOfBytes
import com.radixdlt.sargon.extensions.fromEncryptedJson
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.extensions.toEncryptedJson
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ProfileTest : SampleTestable<Profile> {

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
            deviceFactorSource = hdFactorSource.factorSource.asGeneral(),
            creatingDeviceName = "Unit tests"
        )

        assertEquals("Unit tests - Android", profile.header.creatingDevice.description)
    }

    @Test
    fun testRoundtrip() {
        val sut = Profile.sample()

        assertEquals(
            sut,
            Profile.fromJson(jsonString = sut.toJson())
        )
    }

    @Test
    fun testInitFromMalformedJson() {
        val json = "{}"

        val result =
            runCatching { Profile.fromJson(jsonString = json) }.exceptionOrNull()
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

    @Test
    fun testEncryptionRoundtrip() {
        val password = "ultra secret"
        val sut = Profile.sample()

        val encrypted = sut.toEncryptedJson(encryptionPassword = password)
        val decrypted = Profile.fromEncryptedJson(
            jsonString = encrypted,
            decryptionPassword = password
        )

        assertEquals(sut, decrypted)
    }

    @Test
    fun testAnalyzeContentsOfPlaintextProfile() {
        val sut = Profile.sample()

        val plaintext = sut.toJson()
        assertEquals(
            ProfileFileContents.PlaintextProfile(sut),
            Profile.analyzeContentsOfFile(plaintext)
        )
    }

    @Test
    fun testAnalyzeContentsOfEncryptedProfile() {
        val sut = Profile.sample()

        val encrypted = sut.toEncryptedJson(encryptionPassword = "Super Secret")
        assertEquals(
            ProfileFileContents.EncryptedProfile,
            Profile.analyzeContentsOfFile(encrypted)
        )
    }

    @Test
    fun testAnalyzeContentsOfRandomFile() {
        assertEquals(
            ProfileFileContents.NotProfile,
            Profile.analyzeContentsOfFile(randomBagOfBytes(32).string)
        )
    }
}