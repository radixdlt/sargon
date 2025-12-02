package com.radixdlt.sargon.os.storage.key

import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.toByteArray
import com.radixdlt.sargon.mnemonicWithPassphraseToJsonBytes
import com.radixdlt.sargon.newMnemonicWithPassphraseFromJsonBytes
import com.radixdlt.sargon.os.driver.BiometricAuthorizationDriver
import com.radixdlt.sargon.os.storage.EncryptionHelper
import com.radixdlt.sargon.os.storage.KeySpec
import com.radixdlt.sargon.os.storage.KeystoreAccessRequest
import com.radixdlt.sargon.os.storage.read
import com.radixdlt.sargon.samples.sample
import io.mockk.coEvery
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkConstructor
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
import javax.crypto.Cipher
import javax.crypto.spec.SecretKeySpec

class DeviceFactorSourceMnemonicKeyMappingTest {

    private val testDispatcher = StandardTestDispatcher()
    private val testScope = TestScope(testDispatcher + Job())

    private val mnemonicWithPassphrase = MnemonicWithPassphrase.sample()

    @field:TempDir
    lateinit var tmpDir: File

    private val storage = PreferenceDataStoreFactory.create(scope = testScope) {
        File(tmpDir, "test.preferences_pb")
    }

    @Test
    fun testRoundtrip() = runTest(context = testDispatcher) {
        val driver = TestBiometricAuthorizationDriver(shouldAuthorize = true)
        val mockedMnemonicRequest = mockMnemonicRequest(driver)
        mockkConstructor(KeystoreAccessRequest.ForMnemonic::class)
        every { anyConstructed<KeystoreAccessRequest.ForMnemonic>().keySpec } returns mockedMnemonicRequest.keySpec
        coEvery { anyConstructed<KeystoreAccessRequest.ForMnemonic>().requestAuthorization() } returns Result.success(Unit)

        val factorSourceId = FactorSourceIdFromHash(
            kind = FactorSourceKind.DEVICE,
            body = Exactly32Bytes.sample()
        )
        val sut = DeviceFactorSourceMnemonicKeyMapping(
            key = SecureStorageKey.DeviceFactorSourceMnemonic(
                factorSourceId = factorSourceId
            ),
            encryptedStorage = storage,
            biometricAuthorizationDriver = driver
        )

        val writeResult = sut.write(mnemonicWithPassphraseToJsonBytes(mnemonicWithPassphrase))
        assertTrue(writeResult.isSuccess)

        val readResult = sut.read()
        assertEquals(
            mnemonicWithPassphrase,
            newMnemonicWithPassphraseFromJsonBytes(readResult.getOrThrow()!!)
        )

        // Tests a read directly from storage
        // In order to also assert the name of the key is "mnemonic<hex>" for compatibility
        val readDirectlyFromStorage = storage.read(
            key = stringPreferencesKey("mnemonic${factorSourceId.body.hex}"),
            keystoreAccessRequest = mockedMnemonicRequest
        )
        assertEquals(
            mnemonicWithPassphrase,
            MnemonicWithPassphrase.fromJson(readDirectlyFromStorage.getOrThrow()!!)
        )

        val removeResult = sut.remove()
        assertTrue(removeResult.isSuccess)
        val readResultWhenRemoved = sut.read()
        assertNull(readResultWhenRemoved.getOrThrow())
        assertFalse(sut.keyExist())
    }

    private class TestBiometricAuthorizationDriver(
        private val shouldAuthorize: Boolean
    ): BiometricAuthorizationDriver {

        override val hasStrongAuthenticator: Boolean = false

        override suspend fun authorize(cipher: Cipher?): Result<Cipher?> = if (shouldAuthorize) {
            Result.success(null)
        } else {
            Result.failure(RuntimeException("Authorization denied in this unit test"))
        }
    }

    private fun mockMnemonicRequest(
        driver: BiometricAuthorizationDriver
    ): KeystoreAccessRequest.ForMnemonic {
        val mockKeySpec = mockk<KeySpec.Mnemonic>()
        every { mockKeySpec.getOrGenerateSecretKey() } returns Result.success(
            SecretKeySpec(
                randomBagOfBytes(32).toByteArray(),
                EncryptionHelper.AES_ALGORITHM
            )
        )

        val mockAccessRequest = mockk<KeystoreAccessRequest.ForMnemonic>()
        every { mockAccessRequest.keySpec } returns mockKeySpec
        coEvery { mockAccessRequest.requestAuthorization(any()) } coAnswers { driver.authorize(null).map { KeystoreAccessRequest.AuthorizationArgs.TimeWindowAuth } }
        return mockAccessRequest
    }
}