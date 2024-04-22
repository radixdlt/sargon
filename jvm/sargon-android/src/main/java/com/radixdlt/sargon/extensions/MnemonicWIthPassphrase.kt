package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.HierarchicalDeterministicPublicKey
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.mnemonicWithPassphraseToJsonBytes
import com.radixdlt.sargon.mnemonicWithPassphraseValidatePublicKeys
import com.radixdlt.sargon.newMnemonicWithPassphraseFromJsonBytes

@Throws(SargonException::class)
fun MnemonicWithPassphrase.Companion.deserializeFromBytes(jsonBytes: BagOfBytes) =
    newMnemonicWithPassphraseFromJsonBytes(jsonBytes = jsonBytes)

@Throws(SargonException::class)
fun MnemonicWithPassphrase.Companion.deserializeFromString(fromJson: String) =
    deserializeFromBytes(bagOfBytes(fromString = fromJson))

fun MnemonicWithPassphrase.serializedBytes(): BagOfBytes =
    mnemonicWithPassphraseToJsonBytes(mnemonicWithPassphrase = this)

fun MnemonicWithPassphrase.serializedString(): String = serializedBytes().string

fun MnemonicWithPassphrase.validate(hdPublicKeys: List<HierarchicalDeterministicPublicKey>): Boolean =
    mnemonicWithPassphraseValidatePublicKeys(mnemonicWithPassphrase = this, hdKeys = hdPublicKeys)