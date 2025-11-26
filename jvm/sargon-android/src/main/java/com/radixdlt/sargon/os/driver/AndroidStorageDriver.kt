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
import com.radixdlt.sargon.os.storage.key.DeviceFactorSourceMnemonicKeyMapping
import com.radixdlt.sargon.os.storage.key.FactorSourceUserHasWrittenDownKeyMapping
import com.radixdlt.sargon.os.storage.key.HostIdKeyMapping
import com.radixdlt.sargon.os.storage.key.ProfileSnapshotKeyMapping

internal class AndroidStorageDriver(
    private val biometricAuthorizationDriver: BiometricAuthorizationDriver,
    private val encryptedPreferencesDatastore: DataStore<Preferences>,
    private val preferencesDatastore: DataStore<Preferences>,
    private val deviceInfoDatastore: DataStore<Preferences>
) : SecureStorageDriver, UnsafeStorageDriver {

    override suspend fun loadData(key: SecureStorageKey): BagOfBytes? = key
        .mapping()
        .then { it.read() }
        .reportSecureStorageReadFailure(key = key)
        .getOrNull()

    override suspend fun saveData(key: SecureStorageKey, data: BagOfBytes) {
        key.mapping()
            .then { it.write(data) }
            .reportSecureStorageWriteFailure(key = key)
    }

    override suspend fun deleteDataForKey(key: SecureStorageKey) {
        key.mapping()
            .then { it.remove() }
            .reportSecureStorageWriteFailure(key = key)
    }

    override suspend fun containsDataForKey(key: SecureStorageKey): Boolean = key
        .mapping()
        .getOrNull()
        ?.keyExist()
        ?: false

    override suspend fun loadData(key: UnsafeStorageKey): BagOfBytes? = key
        .mapping()
        .then { it.read() }
        .reportUnsafeStorageReadFailure()
        .getOrNull()

    override suspend fun saveData(key: UnsafeStorageKey, data: BagOfBytes) {
        key.mapping()
            .then { it.write(data) }
            .reportUnsafeStorageWriteFailure()
    }

    override suspend fun deleteDataForKey(key: UnsafeStorageKey) {
        key.mapping()
            .then { it.remove() }
            .reportUnsafeStorageWriteFailure()
    }

    private fun SecureStorageKey.mapping() = when (this) {
        is SecureStorageKey.ProfileSnapshot -> ProfileSnapshotKeyMapping(
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
        UnsafeStorageKey.FACTOR_SOURCE_USER_HAS_WRITTEN_DOWN ->
            FactorSourceUserHasWrittenDownKeyMapping(
                storage = preferencesDatastore
            )
    }.let { mapping ->
        Result.success(mapping)
    }

    private fun <T> Result<T>.reportSecureStorageReadFailure(
        key: SecureStorageKey
    ) = onFailure { error ->
        throw when (error) {
            is BiometricsFailure -> error.toCommonException(key)
            is CommonException -> error
            else -> CommonException.SecureStorageReadException(error.toString())
        }
    }

    private fun <T> Result<T>.reportSecureStorageWriteFailure(
        key: SecureStorageKey
    ) = onFailure { error ->
        throw when (error) {
            is BiometricsFailure -> error.toCommonException(key)
            is CommonException -> error
            else -> CommonException.SecureStorageWriteException(error.toString())
        }
    }

    private fun <T> Result<T>.reportUnsafeStorageReadFailure() = onFailure { error ->
        throw when (error) {
            is CommonException -> error
            else -> CommonException.UnsafeStorageReadException()
        }
    }

    private fun <T> Result<T>.reportUnsafeStorageWriteFailure() = onFailure { error ->
        throw when (error) {
            is CommonException -> error
            else -> CommonException.UnsafeStorageWriteException()
        }
    }
}