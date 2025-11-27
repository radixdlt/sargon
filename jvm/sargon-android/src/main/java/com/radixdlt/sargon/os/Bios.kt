package com.radixdlt.sargon.os

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import com.radixdlt.sargon.ArculusCsdkDriver
import com.radixdlt.sargon.Bios
import com.radixdlt.sargon.Drivers
import com.radixdlt.sargon.NfcTagDriver
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.os.driver.AndroidBiometricAuthorizationDriver
import com.radixdlt.sargon.os.driver.AndroidEntropyProviderDriver
import com.radixdlt.sargon.os.driver.AndroidEventBusDriver
import com.radixdlt.sargon.os.driver.AndroidFileSystemDriver
import com.radixdlt.sargon.os.driver.AndroidHostInfoDriver
import com.radixdlt.sargon.os.driver.AndroidLoggingDriver
import com.radixdlt.sargon.os.driver.AndroidNetworkingDriver
import com.radixdlt.sargon.os.driver.AndroidProfileStateChangeDriver
import com.radixdlt.sargon.os.driver.AndroidStorageDriver
import com.radixdlt.sargon.os.driver.BiometricsHandler
import okhttp3.OkHttpClient

@KoverIgnore
fun Bios.Companion.from(
    context: Context,
    httpClient: OkHttpClient,
    biometricsHandler: BiometricsHandler,
    encryptedPreferencesDataStore: DataStore<Preferences>,
    preferencesDatastore: DataStore<Preferences>,
    deviceInfoDatastore: DataStore<Preferences>,
    eventBusDriver: AndroidEventBusDriver,
    profileStateChangeDriver: AndroidProfileStateChangeDriver,
    arculusCsdkDriver: ArculusCsdkDriver,
    nfcTagDriver: NfcTagDriver
): Bios {
    val storageDriver = AndroidStorageDriver(
        biometricAuthorizationDriver = AndroidBiometricAuthorizationDriver(
            biometricsHandler = biometricsHandler
        ),
        encryptedPreferencesDatastore = encryptedPreferencesDataStore,
        preferencesDatastore = preferencesDatastore,
        deviceInfoDatastore = deviceInfoDatastore,
    )
    return Bios(
        drivers = Drivers(
            networking = AndroidNetworkingDriver(client = httpClient),
            secureStorage = storageDriver,
            unsafeStorage = storageDriver,
            entropyProvider = AndroidEntropyProviderDriver(),
            hostInfo = AndroidHostInfoDriver(context),
            logging = AndroidLoggingDriver(),
            eventBus = eventBusDriver,
            fileSystem = AndroidFileSystemDriver(context),
            profileStateChangeDriver = profileStateChangeDriver,
            arculusCsdkDriver = arculusCsdkDriver,
            nfcTagDriver = nfcTagDriver
        )
    )
}