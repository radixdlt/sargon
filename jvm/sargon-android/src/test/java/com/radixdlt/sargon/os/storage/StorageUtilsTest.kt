package com.radixdlt.sargon.os.storage

import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.toByteArray
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkObject
import io.mockk.mockkStatic
import io.mockk.slot
import kotlinx.coroutines.Job
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestScope
import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Assertions.assertInstanceOf
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.io.TempDir
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.ValueSource
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

    @ParameterizedTest
    @ValueSource(booleans = [false, true])
    fun genericHelpersRejectMnemonicRequestsWithoutAccessingStorage(write: Boolean) = runTest(context = testDispatcher) {
        val storage = object : DataStore<Preferences> {
            override val data = flow<Preferences> { error("Rejected reads must not access preferences") }
            override suspend fun updateData(transform: suspend (Preferences) -> Preferences): Preferences =
                error("Rejected writes must not change preferences")
        }
        val request: KeystoreAccessRequest = KeystoreAccessRequest.ForMnemonic(
            hasStrongAuthenticator = false,
            authorize = { error("Rejected requests must not authorize") }
        )
        val key = stringPreferencesKey("mnemonic-test")

        val result = if (write) storage.write(key, "seed", request) else storage.read(key, request)

        assertInstanceOf(IllegalArgumentException::class.java, result.exceptionOrNull())
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
