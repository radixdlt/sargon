package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DerivationPath
import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.HierarchicalDeterministicPublicKey
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.SignatureWithPublicKey
import com.radixdlt.sargon.mnemonicWithPassphraseDerivePublicKeys
import com.radixdlt.sargon.mnemonicWithPassphraseSign
import com.radixdlt.sargon.mnemonicWithPassphraseToJsonBytes
import com.radixdlt.sargon.mnemonicWithPassphraseValidatePublicKeys
import com.radixdlt.sargon.newMnemonicWithPassphraseFromJsonBytes

@Throws(SargonException::class)
fun MnemonicWithPassphrase.Companion.fromJson(fromJson: String) =
    newMnemonicWithPassphraseFromJsonBytes(bagOfBytes(fromString = fromJson))

fun MnemonicWithPassphrase.toJson(): String =
    mnemonicWithPassphraseToJsonBytes(mnemonicWithPassphrase = this).string

fun MnemonicWithPassphrase.validate(hdPublicKeys: List<HierarchicalDeterministicPublicKey>): Boolean =
    mnemonicWithPassphraseValidatePublicKeys(mnemonicWithPassphrase = this, hdKeys = hdPublicKeys)

fun MnemonicWithPassphrase.derivePublicKey(
    path: DerivationPath
): HierarchicalDeterministicPublicKey = derivePublicKeys(
    paths = listOf(path)
).first()


fun MnemonicWithPassphrase.derivePublicKeys(
    paths: List<DerivationPath>
): List<HierarchicalDeterministicPublicKey> = mnemonicWithPassphraseDerivePublicKeys(
    mnemonicWithPassphrase = this,
    derivationPaths = paths
)

fun MnemonicWithPassphrase.sign(
    hash: Hash,
    path: DerivationPath
): SignatureWithPublicKey = mnemonicWithPassphraseSign(
    mnemonicWithPassphrase = this,
    derivationPath = path,
    hashToSign = hash
)