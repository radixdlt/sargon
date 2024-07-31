package com.radixdlt.sargon.os.radixconnect

import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import com.radixdlt.sargon.SessionId
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.os.storage.EncryptionHelper
import com.radixdlt.sargon.os.storage.KeySpec
import io.mockk.every
import io.mockk.mockkConstructor
import kotlinx.coroutines.Job
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestScope
import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.io.TempDir
import java.io.File
import javax.crypto.spec.SecretKeySpec

@OptIn(ExperimentalUnsignedTypes::class)
class RadixConnectSessionStorageTest {

    private val testDispatcher = StandardTestDispatcher()
    private val testScope = TestScope(testDispatcher + Job())

    @field:TempDir
    lateinit var tmpDir: File

    private val sut = RadixConnectSessionStorage(
        dataStore = PreferenceDataStoreFactory.create(scope = testScope) {
            File(tmpDir, "radix_connect_session_storage.preferences_pb")
        }
    )

    @Test
    fun testRoundtrip() = runTest(context = testDispatcher) {
        mockkConstructor(KeySpec.RadixConnect::class)
        every { anyConstructed<KeySpec.RadixConnect>().getOrGenerateSecretKey() } returns Result.success(
            SecretKeySpec(
                randomBagOfBytes(32).toUByteArray().toByteArray(),
                EncryptionHelper.AES_ALGORITHM
            )
        )

        val sessionId = SessionId.randomUUID()
        val sessionBytes = randomBagOfBytes(32)

        assertNull(sut.loadSession(sessionId))
        sut.saveSession(sessionId, sessionBytes)
        assertEquals(sessionBytes, sut.loadSession(sessionId))
    }

    @Test
    fun testGetNullDueToKeySpecException() = runTest(context = testDispatcher) {
        mockkConstructor(KeySpec.RadixConnect::class)
        every { anyConstructed<KeySpec.RadixConnect>().getOrGenerateSecretKey() } returns Result.success(
            SecretKeySpec(
                randomBagOfBytes(32).toUByteArray().toByteArray(),
                EncryptionHelper.AES_ALGORITHM
            )
        )

        val sessionId = SessionId.randomUUID()
        val sessionBytes = randomBagOfBytes(32)
        sut.saveSession(sessionId, sessionBytes)

        every { anyConstructed<KeySpec.RadixConnect>().getOrGenerateSecretKey() } returns Result.failure(
            RuntimeException("Some Error")
        )
        assertNull(sut.loadSession(sessionId))
    }
}