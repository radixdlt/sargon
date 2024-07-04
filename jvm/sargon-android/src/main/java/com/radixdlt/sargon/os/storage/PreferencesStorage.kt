package com.radixdlt.sargon.os.storage

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.map

internal class PreferencesStorage(
    private val datastore: DataStore<Preferences>
) {

    suspend fun <T : Any> get(key: Preferences.Key<T>): Result<T?> = runCatching {
        datastore.data.catchIOException().map { preferences -> preferences[key] }.first()
    }

    suspend fun <T : Any> set(
        key: Preferences.Key<T>,
        value: T
    ) = datastore.edit { preferences ->
        preferences[key] = value
    }

    suspend fun <T : Any> remove(key: Preferences.Key<T>) {
        datastore.edit { preferences ->
            preferences.remove(key)
        }
    }
}