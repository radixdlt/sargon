package com.radixdlt.sargon.os.driver

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.SecureStorageDriver
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.UnsafeStorageDriver
import com.radixdlt.sargon.UnsafeStorageKey
import com.radixdlt.sargon.extensions.then
import com.radixdlt.sargon.os.storage.key.ByteArrayKeyMapping
import com.radixdlt.sargon.os.storage.key.DeviceFactorSourceMnemonicKeyMapping
import com.radixdlt.sargon.os.storage.key.HostIdKeyMapping
import com.radixdlt.sargon.os.storage.key.ProfileSnapshotKeyMapping
import timber.log.Timber

internal class AndroidStorageDriver(
    private val biometricAuthorizationDriver: BiometricAuthorizationDriver,
    private val encryptedPreferencesDatastore: DataStore<Preferences>,
    private val preferencesDatastore: DataStore<Preferences>,
    private val deviceInfoDatastore: DataStore<Preferences>
) : SecureStorageDriver, UnsafeStorageDriver {

    override suspend fun loadData(key: SecureStorageKey): BagOfBytes? = key
        .mapping()
        .then { it.read() }
        .reportFailure(
            "Failed to load data for $key",
            CommonException.SecureStorageReadException()
        )
        .getOrNull()

    override suspend fun saveData(key: SecureStorageKey, data: BagOfBytes) {
        key.mapping()
            .then { it.write(data) }
            .reportFailure(
                "Failed to save data for $key",
                CommonException.SecureStorageWriteException()
            )
    }

    override suspend fun deleteDataForKey(key: SecureStorageKey) {
        key.mapping()
            .then { it.remove() }
            .reportFailure(
                "Failed to remove data for $key",
                CommonException.SecureStorageWriteException()
            )
    }

    override suspend fun loadData(key: UnsafeStorageKey): BagOfBytes? = key
        .mapping()
        .then { it.read() }
        .reportFailure(
            "Failed to load data for $key",
            CommonException.UnsafeStorageReadException()
        )
        .getOrNull()

    override suspend fun saveData(key: UnsafeStorageKey, data: BagOfBytes) {
        key.mapping()
            .then { it.write(data) }
            .reportFailure(
                "Failed to save data for $key",
                CommonException.UnsafeStorageWriteException()
            )
    }

    override suspend fun deleteDataForKey(key: UnsafeStorageKey) {
        key.mapping()
            .then { it.remove() }
            .reportFailure(
                "Failed to remove data for $key",
                CommonException.UnsafeStorageWriteException()
            )
    }

    private fun SecureStorageKey.mapping() = when (this) {
        is SecureStorageKey.ProfileSnapshot -> ProfileSnapshotKeyMapping(
            key = this,
            encryptedStorage = encryptedPreferencesDatastore
        )

        is SecureStorageKey.HostId -> HostIdKeyMapping(
            key = this,
            deviceStorage = deviceInfoDatastore
        )

        is SecureStorageKey.DeviceFactorSourceMnemonic -> DeviceFactorSourceMnemonicKeyMapping(
            key = this,
            encryptedStorage = encryptedPreferencesDatastore,
            biometricAuthorizationDriver = biometricAuthorizationDriver
        )
    }.let { mapping ->
        Result.success(mapping)
    }

    private fun UnsafeStorageKey.mapping() = when (this) {
        UnsafeStorageKey.FACTOR_SOURCE_USER_HAS_WRITTEN_DOWN -> ByteArrayKeyMapping(
            key = this,
            storage = preferencesDatastore
        )
    }.let { mapping ->
        Result.success(mapping)
    }

    private fun <T> Result<T>.reportFailure(message: String, commonError: CommonException) =
        onFailure { error ->
            Timber.tag("Sargon").w(error, message)
            when (error) {
                is CommonException -> throw error
                else -> throw commonError
            }
        }
}