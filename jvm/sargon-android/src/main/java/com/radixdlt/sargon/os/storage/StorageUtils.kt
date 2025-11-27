package com.radixdlt.sargon.os.storage

import android.util.Base64
import androidx.datastore.core.DataStore
import androidx.datastore.core.IOException
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.emptyPreferences
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.extensions.then
import com.radixdlt.sargon.extensions.toUnit
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.firstOrNull
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.retryWhen

private suspend fun KeystoreAccessRequest?.requestAuthorizationIfNeeded() =
    this?.requestAuthorization() ?: Result.success(null)

/**
 * Reads the contents associated with the given [key] from the data store.
 * If a [KeystoreAccessRequest] is provided then the data written will be decrypted using keystore
 */
suspend fun <T> DataStore<Preferences>.read(
    key: Preferences.Key<T>,
    keystoreAccessRequest: KeystoreAccessRequest? = null,
    retryWhen: suspend ((Throwable, Long) -> Boolean) = { _, _ -> false }
): Result<T?> = keystoreAccessRequest
    .requestAuthorizationIfNeeded()
    .mapCatching {
        data
            .retryWhen { cause, attempt -> retryWhen(cause, attempt) }
            .catchIOException()
            .map { preferences -> preferences[key] }.firstOrNull()
    }.then { value ->
        if (keystoreAccessRequest != null && value != null) {
            value.decrypt(keystoreAccessRequest.keySpec)
        } else {
            Result.success(value)
        }
    }

/**
 * Associates the [value] with the given [key] to the data store.
 * If a [KeystoreAccessRequest] is provided then the data will be encrypted using keystore
 */
suspend fun <T> DataStore<Preferences>.write(
    key: Preferences.Key<T>,
    value: T,
    keystoreAccessRequest: KeystoreAccessRequest? = null
): Result<Unit> = keystoreAccessRequest.requestAuthorizationIfNeeded().then {
    if (keystoreAccessRequest != null && value != null) {
        value.encrypt(keystoreAccessRequest.keySpec)
    } else {
        Result.success(value)
    }
}.mapCatching { modified ->
    edit { preferences ->
        preferences[key] = modified
    }
}.toUnit()

/**
 * Reads the contents associated with the given [key] from the data store,
 * decrypting it using the provided [KeystoreAccessRequest.ForMnemonic].
 */
suspend fun <T> DataStore<Preferences>.read(
    key: Preferences.Key<T>,
    keystoreAccessRequest: KeystoreAccessRequest.ForMnemonic,
    retryWhen: suspend ((Throwable, Long) -> Boolean) = { _, _ -> false }
): Result<T?> = data
    .retryWhen { cause, attempt -> retryWhen(cause, attempt) }
    .catchIOException()
    .map { preferences -> preferences[key] }
    .firstOrNull()
    ?.let { encryptedValue ->
        val encryptedValueByteArray = when (encryptedValue) {
            is String -> Base64.decode(encryptedValue, Base64.DEFAULT)

            is ByteArray -> encryptedValue
            else -> return@let Result.failure(
                IllegalArgumentException(
                    "Attempted to decrypt unsupported type ${this::class.java}"
                )
            )
        }

        keystoreAccessRequest.requestAuthorization(
            purpose = KeystoreAccessRequest.Purpose.OneTimeDecrypt(
                encryptedValue = encryptedValueByteArray
            )
        ).then { args ->
            when (args) {
                is KeystoreAccessRequest.AuthorizationArgs.Decrypt -> encryptedValue.decrypt(
                    keySpec = keystoreAccessRequest.keySpec
                )

                is KeystoreAccessRequest.AuthorizationArgs.Encrypt -> error("Unexpected encrypt args for decryption")

                KeystoreAccessRequest.AuthorizationArgs.TimeWindowAuth -> encryptedValue.decrypt(
                    keySpec = keystoreAccessRequest.keySpec
                )
            }
        }
    } ?: Result.success(null)

/**
 * Associates the [value] with the given [key] to the data store,
 * encrypting it using the provided [KeystoreAccessRequest.ForMnemonic].
 */
suspend fun <T> DataStore<Preferences>.write(
    key: Preferences.Key<T>,
    value: T,
    keystoreAccessRequest: KeystoreAccessRequest.ForMnemonic
): Result<Unit> = if (value != null) {
    keystoreAccessRequest.requestAuthorization(
        purpose = KeystoreAccessRequest.Purpose.OneTimeEncrypt
    ).then { args ->
        when (args) {
            is KeystoreAccessRequest.AuthorizationArgs.Encrypt -> value.encrypt(
                cipher = args.cipher,
                ivBytes = args.ivBytes
            )

            is KeystoreAccessRequest.AuthorizationArgs.Decrypt -> error("Unexpected decrypt args for encryption")

            KeystoreAccessRequest.AuthorizationArgs.TimeWindowAuth -> value.encrypt(
                keySpec = keystoreAccessRequest.keySpec
            )
        }
    }
} else {
    Result.success(value)
}.mapCatching { modified ->
    edit { preferences ->
        preferences[key] = modified
    }
}.toUnit()

suspend fun <T> DataStore<Preferences>.remove(key: Preferences.Key<T>) = runCatching {
    edit { preferences ->
        preferences.remove(key)
    }
}.toUnit()

suspend fun <T> DataStore<Preferences>.keyExist(key: Preferences.Key<T>) = this.data.map { preference ->
    preference.contains(key)
}.first()

@KoverIgnore
internal fun Flow<Preferences>.catchIOException() = catch { exception ->
    if (exception is IOException) {
        emit(emptyPreferences())
    } else {
        throw exception
    }
}