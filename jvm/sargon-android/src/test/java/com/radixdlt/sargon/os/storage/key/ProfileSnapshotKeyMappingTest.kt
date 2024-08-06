package com.radixdlt.sargon.os.storage.key

import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.toBagOfBytes
import com.radixdlt.sargon.extensions.toByteArray
import com.radixdlt.sargon.newProfileFromJsonString
import com.radixdlt.sargon.os.storage.EncryptionHelper
import com.radixdlt.sargon.os.storage.KeySpec
import com.radixdlt.sargon.os.storage.KeystoreAccessRequest
import com.radixdlt.sargon.os.storage.read
import com.radixdlt.sargon.profileToJsonString
import com.radixdlt.sargon.samples.sample
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkObject
import kotlinx.coroutines.Job
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestScope
import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.io.TempDir
import java.io.File
import javax.crypto.spec.SecretKeySpec

class ProfileSnapshotKeyMappingTest {

    private val testDispatcher = StandardTestDispatcher()
    private val testScope = TestScope(testDispatcher + Job())

    private val profile = Profile.sample()

    @field:TempDir
    lateinit var tmpDir: File

    private val storage = PreferenceDataStoreFactory.create(scope = testScope) {
        File(tmpDir, "test.preferences_pb")
    }

    @Test
    fun testRoundtrip() = runTest(context = testDispatcher) {
        mockProfileAccessRequest()

        val sut = ProfileSnapshotKeyMapping(
            key = SecureStorageKey.ProfileSnapshot,
            encryptedStorage = storage
        )

        val writeResult = sut.write(profileToJsonString(profile, false).toByteArray().toBagOfBytes())
        assertTrue(writeResult.isSuccess)

        val readResult = sut.read()
        assertEquals(
            profile,
            newProfileFromJsonString(readResult.getOrThrow()!!.string)
        )

        // Tests a read directly from storage
        // In order to also assert the name of the key is "profile_preferences_key" for compatibility
        val readDirectlyFromStorage = storage.read(
            key = stringPreferencesKey("profile_preferences_key"),
            keystoreAccessRequest = KeystoreAccessRequest.ForProfile
        )
        assertEquals(
            profile,
            newProfileFromJsonString(readDirectlyFromStorage.getOrThrow()!!)
        )

        val removeResult = sut.remove()
        assertTrue(removeResult.isSuccess)
        val readResultWhenRemoved = sut.read()
        assertNull(readResultWhenRemoved.getOrThrow())
    }

    private fun mockProfileAccessRequest() {
        val mockKeySpec = mockk<KeySpec.Profile>()
        every { mockKeySpec.getOrGenerateSecretKey() } returns Result.success(
            SecretKeySpec(
                randomBagOfBytes(32).toByteArray(),
                EncryptionHelper.AES_ALGORITHM
            )
        )

        mockkObject(KeystoreAccessRequest.ForProfile)
        every { KeystoreAccessRequest.ForProfile.keySpec } returns mockKeySpec
    }
}