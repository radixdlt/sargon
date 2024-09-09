package com.radixdlt.sargon.os.driver

import androidx.biometric.BiometricPrompt
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.SecureStorageAccessErrorKind
import com.radixdlt.sargon.SecureStorageDriver
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.UnsafeStorageDriver
import com.radixdlt.sargon.UnsafeStorageKey
import com.radixdlt.sargon.extensions.then
import com.radixdlt.sargon.os.storage.key.ByteArrayKeyMapping
import com.radixdlt.sargon.os.storage.key.DeviceFactorSourceMnemonicKeyMapping
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

    private fun <T> Result<T>.reportSecureStorageReadFailure(
        key: SecureStorageKey
    ) = onFailure { error ->
        throw when (error) {
            is BiometricsFailure -> error.toCommonException(key)
            is CommonException -> error
            else -> CommonException.SecureStorageReadException()
        }
    }

    private fun <T> Result<T>.reportSecureStorageWriteFailure(
        key: SecureStorageKey
    ) = onFailure { error ->
        throw when (error) {
            is BiometricsFailure -> error.toCommonException(key)
            is CommonException -> error
            else -> CommonException.SecureStorageWriteException()
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

    private fun BiometricsFailure.toCommonException(key: SecureStorageKey) = CommonException.SecureStorageAccessException(
        key = key,
        errorKind = when (errorCode) {
            BiometricPrompt.ERROR_CANCELED -> SecureStorageAccessErrorKind.CANCELLED
            BiometricPrompt.ERROR_HW_NOT_PRESENT -> SecureStorageAccessErrorKind.HARDWARE_NOT_PRESENT
            BiometricPrompt.ERROR_HW_UNAVAILABLE -> SecureStorageAccessErrorKind.HARDWARE_UNAVAILABLE
            BiometricPrompt.ERROR_LOCKOUT -> SecureStorageAccessErrorKind.LOCKOUT
            BiometricPrompt.ERROR_LOCKOUT_PERMANENT -> SecureStorageAccessErrorKind.LOCKOUT_PERMANENT
            BiometricPrompt.ERROR_NEGATIVE_BUTTON -> SecureStorageAccessErrorKind.NEGATIVE_BUTTON
            BiometricPrompt.ERROR_NO_BIOMETRICS -> SecureStorageAccessErrorKind.NO_BIOMETRICS
            BiometricPrompt.ERROR_NO_DEVICE_CREDENTIAL -> SecureStorageAccessErrorKind.NO_DEVICE_CREDENTIAL
            BiometricPrompt.ERROR_NO_SPACE -> SecureStorageAccessErrorKind.NO_SPACE
            BiometricPrompt.ERROR_TIMEOUT -> SecureStorageAccessErrorKind.TIMEOUT
            BiometricPrompt.ERROR_UNABLE_TO_PROCESS -> SecureStorageAccessErrorKind.UNABLE_TO_PROCESS
            BiometricPrompt.ERROR_USER_CANCELED -> SecureStorageAccessErrorKind.USER_CANCELLED
            BiometricPrompt.ERROR_VENDOR -> SecureStorageAccessErrorKind.VENDOR
            else -> throw CommonException.Unknown()
        },
        errorMessage = errorMessage.orEmpty()
    )
}