package com.radixdlt.sargon.os.homecards

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.byteArrayPreferencesKey
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.HomeCardsStorage
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.extensions.toBagOfBytes
import com.radixdlt.sargon.os.storage.PreferencesStorage

internal class HomeCardsStorageImpl internal constructor(
    private val storage: PreferencesStorage
) : HomeCardsStorage {

    @KoverIgnore
    constructor(dataStore: DataStore<Preferences>) : this(
        storage = PreferencesStorage(
            datastore = dataStore
        )
    )

    override suspend fun saveCards(encodedCards: BagOfBytes) {
        storage.set(KEY_HOME_CARDS, encodedCards.toUByteArray().toByteArray())
    }

    override suspend fun loadCards(): BagOfBytes? {
        return storage.get(KEY_HOME_CARDS).getOrNull()?.toBagOfBytes()
    }

    companion object {
        private val KEY_HOME_CARDS = byteArrayPreferencesKey("home_cards")
    }
}