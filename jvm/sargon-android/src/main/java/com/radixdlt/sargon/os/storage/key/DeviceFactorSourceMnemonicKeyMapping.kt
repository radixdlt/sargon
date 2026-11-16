package com.radixdlt.sargon.os.storage.key

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.then
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.mnemonicWithPassphraseToJsonBytes
import com.radixdlt.sargon.newMnemonicWithPassphraseFromJsonBytes
import com.radixdlt.sargon.os.storage.MnemonicStorage

internal class DeviceFactorSourceMnemonicKeyMapping(
    key: SecureStorageKey.DeviceFactorSourceMnemonic,
    private val mnemonicStorage: MnemonicStorage
) : DatastoreKeyMapping {

    private val preferencesKey = "mnemonic${key.factorSourceId.body.hex}"

    override suspend fun write(bagOfBytes: BagOfBytes): Result<Unit> = runCatching {
        newMnemonicWithPassphraseFromJsonBytes(bagOfBytes).toJson()
    }.then { json ->
        mnemonicStorage.write(preferencesKey, json)
    }

    override suspend fun read(): Result<BagOfBytes?> = mnemonicStorage.read(preferencesKey).mapCatching { androidCompatibleJson ->
        if (androidCompatibleJson != null) {
            val mnemonicWithPassphrase = MnemonicWithPassphrase.fromJson(androidCompatibleJson)
            mnemonicWithPassphraseToJsonBytes(mnemonicWithPassphrase)
        } else {
            null
        }
    }

    override suspend fun remove(): Result<Unit> = mnemonicStorage.remove(preferencesKey)

    override suspend fun keyExist(): Boolean = mnemonicStorage.contains(preferencesKey)
}
