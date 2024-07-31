package com.radixdlt.sargon.os.storage

import androidx.datastore.core.DataStore
import androidx.datastore.core.IOException
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.emptyPreferences
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.extensions.toUnit
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.firstOrNull
import kotlinx.coroutines.flow.map

internal suspend fun <T> DataStore<Preferences>.read(
    key: Preferences.Key<T>,
    keySpec: KeySpec? = null
): Result<T?> {
    val value = data.catchIOException().map { preferences -> preferences[key] }.firstOrNull()

    return if (keySpec != null && value != null) {
        value.decrypt(keySpec)
    } else {
        Result.success(value)
    }
}

internal suspend fun <T> DataStore<Preferences>.write(
    key: Preferences.Key<T>,
    value: T,
    keySpec: KeySpec? = null
) = if (keySpec != null && value != null) {
    value.encrypt(keySpec)
} else {
    Result.success(value)
}.mapCatching { modified ->
    edit { preferences ->
        preferences[key] = modified
    }
}.toUnit()

internal suspend fun <T> DataStore<Preferences>.remove(key: Preferences.Key<T>) = runCatching {
    edit { preferences ->
        preferences.remove(key)
    }
}.toUnit()

@KoverIgnore
internal fun Flow<Preferences>.catchIOException() = catch { exception ->
    if (exception is IOException) {
        emit(emptyPreferences())
    } else {
        throw exception
    }
}