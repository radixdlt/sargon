package com.radixdlt.sargon.os.driver

import android.content.Context
import androidx.datastore.preferences.SharedPreferencesMigration
import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.preferencesDataStoreFile
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.filters.MediumTest
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.ProfileId
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.UnsafeStorageKey
import com.radixdlt.sargon.Uuid
import com.radixdlt.sargon.extensions.bagOfBytes
import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.samples.sample
import junit.framework.TestCase.assertEquals
import junit.framework.TestCase.assertFalse
import junit.framework.TestCase.assertNull
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Test
import org.junit.runner.RunWith
import java.io.File

/**
 * For tests specific to some keys refer to:
 * * [AndroidStorageDriverHostIdTest] for [SecureStorageKey.HostId]
 * * [AndroidStorageDriverDeviceFactorSourceMnemonicTest] for [SecureStorageKey.DeviceFactorSourceMnemonic]
 */
@RunWith(AndroidJUnit4::class)
@MediumTest
class AndroidStorageDriverTest {

    private val testContext: Context = ApplicationProvider.getApplicationContext()

    @After
    fun deleteDatastores() {
        File(testContext.filesDir, "datastore").deleteRecursively()
        testContext.getSharedPreferences(
            OLD_DEVICE_INFO_PREFERENCES,
            Context.MODE_PRIVATE
        ).edit().clear().commit()
    }

    @Test
    fun testProfileKeyWhenExists() = runTest {
        val sut = sut(testContext, backgroundScope)
        val profile = Profile.sample()
        val jsonBytes = bagOfBytes(profile.toJson())
        val key = SecureStorageKey.ProfileSnapshot(profileId = ProfileId.randomUUID())
        sut.saveData(key, jsonBytes)

        val receivedBytes = sut.loadData(key)

        assertEquals(
            profile,
            receivedBytes?.string?.let { Profile.fromJson(it) }
        )
    }

    @Test
    fun testProfileKeyWhenDeleted() = runTest {
        val sut = sut(testContext, backgroundScope)
        val profile = Profile.sample()
        val jsonBytes = bagOfBytes(profile.toJson())
        val key = SecureStorageKey.ProfileSnapshot(profileId = ProfileId.randomUUID())
        sut.saveData(key, jsonBytes)
        assertEquals(
            profile,
            sut.loadData(key)?.string?.let { Profile.fromJson(it) }
        )

        sut.deleteDataForKey(key)

        assertNull(sut.loadData(key))
        assertFalse(sut.containsDataForKey(key))
    }

    @Test
    fun testCrudForByteArrayUnsafeKey() = runTest {
        val sut = sut(testContext, backgroundScope)

        val bytes = randomBagOfBytes(2)
        sut.saveData(UnsafeStorageKey.FACTOR_SOURCE_USER_HAS_WRITTEN_DOWN, bytes)
        assertEquals(
            bytes,
            sut.loadData(UnsafeStorageKey.FACTOR_SOURCE_USER_HAS_WRITTEN_DOWN)
        )
        sut.deleteDataForKey(UnsafeStorageKey.FACTOR_SOURCE_USER_HAS_WRITTEN_DOWN)
        assertNull(
            sut.loadData(UnsafeStorageKey.FACTOR_SOURCE_USER_HAS_WRITTEN_DOWN)
        )
    }

    companion object {
        internal fun sut(
            context: Context,
            scope: CoroutineScope,
            onAuthorize: () -> Result<Unit> = { Result.success(Unit) }
        ) = AndroidStorageDriver(
            encryptedPreferencesDatastore = encryptedDataStore(context, scope),
            preferencesDatastore = unEncryptedDataStore(context, scope),
            deviceInfoDatastore = deviceInfoDataStore(context, scope),
            biometricAuthorizationDriver = TestBiometricAuthorizationDriver(onAuthorize)
        )

        private class TestBiometricAuthorizationDriver(
            private val onAuthorize: () -> Result<Unit>
        ): BiometricAuthorizationDriver {
            override suspend fun authorize(): Result<Unit> = onAuthorize()
        }

        private fun encryptedDataStore(
            context: Context,
            scope: CoroutineScope,
        ) = PreferenceDataStoreFactory.create(
            scope = scope
        ) { context.testDatastoreFile() }

        private fun unEncryptedDataStore(
            context: Context,
            scope: CoroutineScope,
        ) = PreferenceDataStoreFactory.create(
            scope = scope
        ) { context.testDatastoreFile() }

        private fun deviceInfoDataStore(
            context: Context,
            scope: CoroutineScope,
        ) = PreferenceDataStoreFactory.create(
            scope = scope,
            migrations = listOf(SharedPreferencesMigration(context, OLD_DEVICE_INFO_PREFERENCES)),
        ) { context.testDatastoreFile() }

        // Files need to be random in order for tests to run in parallel.
        // Multiple instances of the same file cannot be open at the same time.
        private fun Context.testDatastoreFile() = preferencesDataStoreFile(
            Uuid.randomUUID().toString()
        )

        const val OLD_DEVICE_INFO_PREFERENCES = "device_prefs"
    }

}