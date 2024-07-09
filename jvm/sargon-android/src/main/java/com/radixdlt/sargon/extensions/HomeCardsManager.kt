package com.radixdlt.sargon.extensions

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import com.radixdlt.sargon.HomeCardsManager
import com.radixdlt.sargon.HomeCardsObserver
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.antenna.SargonNetworkAntenna
import com.radixdlt.sargon.os.homecards.HomeCardsStorageImpl
import okhttp3.OkHttpClient

fun HomeCardsManager.Companion.init(
    okHttpClient: OkHttpClient,
    networkId: NetworkId,
    dataStore: DataStore<Preferences>,
    observer: HomeCardsObserver
) = HomeCardsManager(
    networkAntenna = SargonNetworkAntenna(client = okHttpClient),
    networkId = networkId,
    cardsStorage = HomeCardsStorageImpl(dataStore = dataStore),
    observer = observer
)