package com.radixdlt.sargon.os.storage.key

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.then
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.mnemonicWithPassphraseToJsonBytes
import com.radixdlt.sargon.newMnemonicWithPassphraseFromJsonBytes
import com.radixdlt.sargon.os.driver.BiometricAuthorizationDriver
import com.radixdlt.sargon.os.storage.KeySpec
import com.radixdlt.sargon.os.storage.KeystoreAccessRequest
import com.radixdlt.sargon.os.storage.read
import com.radixdlt.sargon.os.storage.remove
import com.radixdlt.sargon.os.storage.write
import timber.log.Timber

internal class DeviceFactorSourceMnemonicKeyMapping(
    private val key: SecureStorageKey.DeviceFactorSourceMnemonic,
    private val encryptedStorage: DataStore<Preferences>,
    private val biometricAuthorizationDriver: BiometricAuthorizationDriver
): DatastoreKeyMapping {

    private val preferencesKey = stringPreferencesKey("mnemonic${key.factorSourceId.body.hex}")

    override suspend fun write(bagOfBytes: BagOfBytes): Result<Unit> = runCatching {
        newMnemonicWithPassphraseFromJsonBytes(bagOfBytes).toJson()
    }.then { json ->
        encryptedStorage.write(
            key = preferencesKey,
            value = json,
            keystoreAccessRequest = KeystoreAccessRequest.ForMnemonic(
                onRequestAuthorization = { biometricAuthorizationDriver.authorize() }
            )
        )
    }

    override suspend fun read(): Result<BagOfBytes?> = encryptedStorage.read(
        key = preferencesKey,
        keystoreAccessRequest = KeystoreAccessRequest.ForMnemonic(
            onRequestAuthorization = { biometricAuthorizationDriver.authorize() }
        )
    ).mapCatching { androidCompatibleJson ->
        if (androidCompatibleJson != null) {
            val mnemonicWithPassphrase = MnemonicWithPassphrase.fromJson(androidCompatibleJson)
            mnemonicWithPassphraseToJsonBytes(mnemonicWithPassphrase)
        } else {
            null
        }
    }

    override suspend fun remove(): Result<Unit> = encryptedStorage.remove(key = preferencesKey)
}