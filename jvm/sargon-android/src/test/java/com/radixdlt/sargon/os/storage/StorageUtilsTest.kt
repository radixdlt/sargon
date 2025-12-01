package com.radixdlt.sargon.os.storage

import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.toByteArray
import io.mockk.coEvery
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkObject
import io.mockk.mockkStatic
import io.mockk.slot
import kotlinx.coroutines.Job
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestScope
import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.io.TempDir
import java.io.File
import javax.crypto.spec.SecretKeySpec
import kotlin.io.encoding.ExperimentalEncodingApi
import android.util.Base64 as AndroidBase64
import kotlin.io.encoding.Base64.Default.Mime as KotlinLikeAndroidBase64

class StorageUtilsTest {

    private val testDispatcher = StandardTestDispatcher()
    private val testScope = TestScope(testDispatcher + Job())

    @field:TempDir
    lateinit var tmpDir: File

    private val sut = PreferenceDataStoreFactory.create(scope = testScope) {
        File(tmpDir, "test.preferences_pb")
    }

    @OptIn(ExperimentalEncodingApi::class)
    @BeforeEach
    fun before() {
        val byteArrayInputSlot = slot<ByteArray>()
        mockkStatic(AndroidBase64::class)
        every {
            AndroidBase64.encodeToString(capture(byteArrayInputSlot), AndroidBase64.DEFAULT)
        } answers {
            KotlinLikeAndroidBase64.encode(byteArrayInputSlot.captured)
        }

        val stringInputSlot = slot<String>()
        every {
            AndroidBase64.decode(capture(stringInputSlot), AndroidBase64.DEFAULT)
        } answers {
            KotlinLikeAndroidBase64.decode(stringInputSlot.captured)
        }
    }

    @Test
    fun testReadWhenNullValueWithoutAuhotize() = runTest(context = testDispatcher) {
        val value = sut.read(
            key = stringPreferencesKey("a_key"),
        )

        assertNull(value.getOrThrow())
    }

    @Test
    fun testRoundtripWithoutAccessRequest() = runTest(context = testDispatcher) {
        sut.write(
            key = stringPreferencesKey("a_key"),
            value = "a value"
        )

        val value = sut.read(
            key = stringPreferencesKey("a_key"),
        )

        assertEquals(
            "a value",
            value.getOrThrow()
        )
    }

    @Test
    fun testReadNullWhenNoValueWhenAuthorized() = runTest(context = testDispatcher) {
        val value = sut.read(
            key = stringPreferencesKey("a_key"),
            keystoreAccessRequest = KeystoreAccessRequest.ForProfile
        )

        assertNull(value.getOrThrow())
    }

    @Test
    fun testRoundtripWhenAlwaysAuthorized() = runTest(context = testDispatcher) {
        mockProfileAccessRequest()

        sut.write(
            key = stringPreferencesKey("a_key"),
            value = "a value",
            keystoreAccessRequest = KeystoreAccessRequest.ForProfile
        ).getOrThrow()

        val value = sut.read(
            key = stringPreferencesKey("a_key"),
            keystoreAccessRequest = KeystoreAccessRequest.ForProfile
        )

        assertEquals(
            "a value",
            value.getOrThrow()
        )
    }

    @Test
    fun testWriteFailWhenNotAuthorized() = runTest(context = testDispatcher) {
        val mnemonicAccessRequest = mockMnemonicRequest(onAuthorizeWhenRequested = false)

        val result = sut.write(
            key = stringPreferencesKey("a_key"),
            value = "a value",
            keystoreAccessRequest = mnemonicAccessRequest
        )

        assertTrue(result.isFailure)
    }

    @Test
    fun testWriteSucceedWhenAuthorized() = runTest(context = testDispatcher) {
        val mnemonicAccessRequest = mockMnemonicRequest(onAuthorizeWhenRequested = true)

        val result = sut.write(
            key = stringPreferencesKey("a_key"),
            value = "a value",
            keystoreAccessRequest = mnemonicAccessRequest
        )

        assertTrue(result.isSuccess)
    }

    @Test
    fun testRoundtripFailWhenNotAuthorizedOnRead() = runTest(context = testDispatcher) {
        val mnemonicAccessRequestApproved = mockMnemonicRequest(onAuthorizeWhenRequested = true)

        val writeResult = sut.write(
            key = stringPreferencesKey("a_key"),
            value = "a value",
            keystoreAccessRequest = mnemonicAccessRequestApproved
        )
        assertTrue(writeResult.isSuccess)

        val mnemonicAccessRequestDenied = mockMnemonicRequest(onAuthorizeWhenRequested = false)
        val readResult = sut.read(
            key = stringPreferencesKey("a_key"),
            keystoreAccessRequest = mnemonicAccessRequestDenied
        )
        assertTrue(readResult.isFailure)
    }

    @Test
    fun testRoundtripWhenAuthorized() = runTest(context = testDispatcher) {
        val mnemonicAccessRequest = mockMnemonicRequest(onAuthorizeWhenRequested = true)

        val writeResult = sut.write(
            key = stringPreferencesKey("a_key"),
            value = "a value",
            keystoreAccessRequest = mnemonicAccessRequest
        )
        assertTrue(writeResult.isSuccess)

        val readResult = sut.read(
            key = stringPreferencesKey("a_key"),
            keystoreAccessRequest = mnemonicAccessRequest
        )
        assertEquals(
            "a value",
            readResult.getOrThrow()
        )
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

    private fun mockMnemonicRequest(
        onAuthorizeWhenRequested: Boolean
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
        coEvery { mockAccessRequest.requestAuthorization(any()) } returns if (onAuthorizeWhenRequested) {
            Result.success(KeystoreAccessRequest.AuthorizationArgs.TimeWindowAuth)
        } else {
            Result.failure(RuntimeException("Not allowed to authorized in this unit test"))
        }
        return mockAccessRequest
    }
}