package com.radixdlt.sargon.os.storage

import androidx.datastore.core.IOException
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.emptyPreferences
import com.radixdlt.sargon.annotation.KoverIgnore
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.catch

@KoverIgnore
internal fun Flow<Preferences>.catchIOException() = catch { exception ->
    if (exception is IOException) {
        emit(emptyPreferences())
    } else {
        throw exception
    }
}