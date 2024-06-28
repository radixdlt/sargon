package com.radixdlt.sargon.os.storage

import androidx.datastore.core.DataStore
import androidx.datastore.core.IOException
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.emptyPreferences
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.extensions.then
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.map

internal class EncryptedPreferencesStorage(
    private val datastore: DataStore<Preferences>
) {

    suspend fun <T : Any> get(key: Preferences.Key<T>, keySpec: KeySpec): Result<T?> = runCatching {
        datastore.data.catchIOException().map { preferences -> preferences[key] }.first()
    }.then { encrypted ->
        if (encrypted == null) return@then Result.success(null)

        encrypted.decrypt(keySpec = keySpec)
    }

    suspend fun <T : Any> set(
        key: Preferences.Key<T>,
        value: T,
        keySpec: KeySpec
    ): Result<Unit> = value.encrypt(keySpec = keySpec).mapCatching { encrypted ->
        datastore.edit { preferences ->
            preferences[key] = encrypted
        }
    }

    suspend fun <T : Any> remove(key: Preferences.Key<T>) {
        datastore.edit { preferences ->
            preferences.remove(key)
        }
    }

    @KoverIgnore
    private fun Flow<Preferences>.catchIOException() = catch { exception ->
        if (exception is IOException) {
            emit(emptyPreferences())
        } else {
            throw exception
        }
    }
}