package com.radixdlt.sargon.os.storage

import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.extensions.randomBagOfBytes
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.Job
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestScope
import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import org.junit.jupiter.api.io.TempDir
import java.io.File
import javax.crypto.spec.SecretKeySpec

@OptIn(ExperimentalUnsignedTypes::class)
class EncryptedPreferencesStorageTest {

    private val testDispatcher = StandardTestDispatcher()
    private val testScope = TestScope(testDispatcher + Job())

    private val mockKeySpec = mockk<KeySpec>().apply {
        every { getOrGenerateSecretKey() } returns Result.success(
            SecretKeySpec(
                randomBagOfBytes(32).toUByteArray().toByteArray(),
                EncryptionHelper.AES_ALGORITHM
            )
        )
    }

    @field:TempDir
    lateinit var tmpDir: File

    private val sut = EncryptedPreferencesStorage(
        datastore = PreferenceDataStoreFactory.create(scope = testScope) {
            File(tmpDir, "test.preferences_pb")
        }
    )

    @Test
    fun testRoundtrip() = runTest(context = testDispatcher) {
        val pair = "some-key" to "Some value to save"
        sut.set(stringPreferencesKey(pair.first), pair.second, mockKeySpec).getOrThrow()

        val value = sut.get(stringPreferencesKey(pair.first), mockKeySpec).getOrThrow()
        assertEquals(pair.second, value)
    }

    @Test
    fun testGetWithNoSetReturnsNull() = runTest(context = testDispatcher) {
        val value = sut.get(stringPreferencesKey("some-other-key"), mockKeySpec).getOrThrow()
        assertNull(value)
    }

    @Test
    fun testGetAfterRemovingKeyReturnsNull() = runTest(context = testDispatcher) {
        val pair = "some-key" to "Some value to save"
        sut.set(stringPreferencesKey(pair.first), pair.second, mockKeySpec).getOrThrow()

        assertEquals(sut.get(stringPreferencesKey(pair.first), mockKeySpec).getOrThrow(), pair.second)

        sut.remove(stringPreferencesKey(pair.first))

        val value = sut.get(stringPreferencesKey("some-other-key"), mockKeySpec).getOrThrow()
        assertNull(value)
    }

    @Test
    fun testEncryptErrorOnSet() = runTest(context = testDispatcher) {
        every { mockKeySpec.getOrGenerateSecretKey() } returns Result.failure(RuntimeException("Some Error"))

        val pair = "some-key" to "Some value to save"
        assertThrows<RuntimeException> {
            sut.set(stringPreferencesKey(pair.first), pair.second, mockKeySpec).getOrThrow()
        }
    }

}