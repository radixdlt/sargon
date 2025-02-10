package com.radixdlt.sargon.os.storage.key

import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import com.radixdlt.sargon.ProfileId
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.UnsafeStorageKey
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.toByteArray
import com.radixdlt.sargon.newProfileIdSample
import com.radixdlt.sargon.os.storage.EncryptionHelper
import com.radixdlt.sargon.os.storage.KeySpec
import com.radixdlt.sargon.os.storage.KeystoreAccessRequest
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkObject
import kotlinx.coroutines.Job
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestScope
import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.io.TempDir
import java.io.File
import javax.crypto.spec.SecretKeySpec

class ByteArrayKeyMappingTest {

    private val testDispatcher = StandardTestDispatcher()
    private val testScope = TestScope(testDispatcher + Job())

    @field:TempDir
    lateinit var tmpDir: File

    private val storage = PreferenceDataStoreFactory.create(scope = testScope) {
        File(tmpDir, "test.preferences_pb")
    }

    @Test
    fun testUnsafeKeyRoundtrip() = runTest(context = testDispatcher) {
        val key = UnsafeStorageKey.FACTOR_SOURCE_USER_HAS_WRITTEN_DOWN

        val sut = ByteArrayKeyMapping(
            key = key,
            storage = storage
        )

        val bytesToStore = randomBagOfBytes(32)
        val writeResult = sut.write(bytesToStore)
        assertTrue(writeResult.isSuccess)

        val bytesRestored = sut.read()
        assertEquals(
            bytesToStore,
            bytesRestored.getOrThrow()
        )

        sut.remove()
        val bytesRestoredAfterRemove = sut.read()
        assertNull(bytesRestoredAfterRemove.getOrThrow())
        assertFalse(sut.keyExist())
    }

    @Test
    fun testSecureStorageKeyRoundtrip() = runTest(context = testDispatcher) {
        // Even thought profile snapshot does not store data in byte array,
        // it is just used to facilitate the test
        val key = SecureStorageKey.ProfileSnapshot(newProfileIdSample())
        mockProfileAccessRequest()

        val sut = ByteArrayKeyMapping(
            key = key,
            keystoreAccessRequest = KeystoreAccessRequest.ForProfile,
            storage = storage
        )

        val bytesToStore = randomBagOfBytes(32)
        val writeResult = sut.write(bytesToStore)
        assertTrue(writeResult.isSuccess)

        val bytesRestored = sut.read()
        assertEquals(
            bytesToStore,
            bytesRestored.getOrThrow()
        )

        sut.remove()
        val bytesRestoredAfterRemove = sut.read()
        assertNull(bytesRestoredAfterRemove.getOrThrow())
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