package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.analyzeContentsOfFile
import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.bagOfBytes
import com.radixdlt.sargon.extensions.checkIfEncryptedProfileJsonContainsLegacyP2PLinks
import com.radixdlt.sargon.extensions.checkIfProfileJsonContainsLegacyP2PLinks
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
import java.io.File

class ProfileTest : SampleTestable<Profile> {

    override val samples: List<Sample<Profile>>
        get() = listOf(Profile.sample)

    @Test
    fun testInit() {
        val hdFactorSource = PrivateHierarchicalDeterministicFactorSource.init(
            isMainBDFS = true,
            entropy = NonEmptyMax32Bytes(bagOfBytes = randomBagOfBytes(byteCount = 32)),
            deviceInfo = DeviceInfo.sample.other()
        )

        val profile = Profile.init(
            deviceFactorSource = hdFactorSource.factorSource.asGeneral(),
            deviceInfo = DeviceInfo.sample.other()
        )

        assertEquals("Android (Android)", profile.header.creatingDevice.description)
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

    @Test
    fun testCheckIfProfileJsonContainsLegacyP2PLinksWhenP2PLinksAreNotPresent() {
        Profile.sample.all.forEach { sut ->
            assertEquals(
                false,
                Profile.checkIfProfileJsonContainsLegacyP2PLinks(sut.toJson())
            )
        }
    }

    @Test
    fun testCheckIfProfileJsonContainsLegacyP2PLinksWhenP2PLinksArePresent() {
        val json = File("../../" + "fixtures/vector/only_plaintext_profile_snapshot_version_100.json").readText()
        assertEquals(
            true,
            Profile.checkIfProfileJsonContainsLegacyP2PLinks(json)
        )
    }

    @Test
    fun testCheckIfEncryptedProfileJsonContainsLegacyP2PLinksWhenEmptyJson() {
        assertEquals(
            false,
            Profile.checkIfEncryptedProfileJsonContainsLegacyP2PLinks("{}", "babylon"),
        )
    }

    @Test
    fun testCheckIfEncryptedProfileJsonContainsLegacyP2PLinksWhenP2PLinksArePresent() {
        val json = File("../../" + "fixtures/vector/profile_encrypted_by_password_of_babylon.json").readText()
        assertEquals(
            true,
            Profile.checkIfEncryptedProfileJsonContainsLegacyP2PLinks(json, "babylon")
        )
    }
}