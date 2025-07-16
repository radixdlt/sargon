package com.radixdlt.sargon.os.homecards

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.byteArrayPreferencesKey
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.HomeCardsStorage
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.extensions.toBagOfBytes
import com.radixdlt.sargon.extensions.toByteArray
import com.radixdlt.sargon.os.storage.read
import com.radixdlt.sargon.os.storage.write

@KoverIgnore
internal class HomeCardsStorageImpl internal constructor(
    private val dataStore: DataStore<Preferences>
) : HomeCardsStorage {

    override suspend fun saveCards(encodedCards: BagOfBytes) {
        dataStore.write(KEY_HOME_CARDS, encodedCards.toByteArray())
    }

    override suspend fun loadCards(): BagOfBytes? {
        return dataStore.read(KEY_HOME_CARDS).getOrNull()?.toBagOfBytes()
    }

    override suspend fun saveDismissedCards(encodedCards: BagOfBytes) {
        dataStore.write(KEY_DISMISSED_HOME_CARDS, encodedCards.toByteArray())
    }

    override suspend fun loadDismissedCards(): BagOfBytes? {
        return dataStore.read(KEY_DISMISSED_HOME_CARDS).getOrNull()?.toBagOfBytes()
    }

    companion object {
        private val KEY_HOME_CARDS = byteArrayPreferencesKey("home_cards")
        private val KEY_DISMISSED_HOME_CARDS = byteArrayPreferencesKey("dismissed_home_cards")
    }
}
