package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Bip39Passphrase
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.DerivationPath
import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.HierarchicalDeterministicFactorInstance
import com.radixdlt.sargon.HierarchicalDeterministicPublicKey
import com.radixdlt.sargon.Mnemonic
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.SignatureWithPublicKey
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.extensions.AndroidMnemonicWithPassphrase.Companion.toAndroid
import com.radixdlt.sargon.mnemonicWithPassphraseDerivePublicKeys
import com.radixdlt.sargon.mnemonicWithPassphraseSign
import com.radixdlt.sargon.mnemonicWithPassphraseValidatePublicKeys
import com.radixdlt.sargon.newFactorSourceIdFromHashFromMnemonicWithPassphrase
import com.radixdlt.sargon.os.signing.HdSignature
import com.radixdlt.sargon.os.signing.HdSignatureInput
import com.radixdlt.sargon.os.signing.PerFactorSourceInput
import com.radixdlt.sargon.os.signing.Signable
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

@Throws(SargonException::class)
fun MnemonicWithPassphrase.Companion.init(phrase: String) = MnemonicWithPassphrase(
    mnemonic = Mnemonic.init(phrase = phrase),
    passphrase = Bip39Passphrase()
)

@Throws(SargonException::class)
fun MnemonicWithPassphrase.Companion.fromJson(
    fromJson: String
) = runCatching {
    Json.decodeFromString<AndroidMnemonicWithPassphrase>(fromJson)
}.map {
    it.toMnemonicWithPassphrase()
}.onFailure {
    throw CommonException.FailedToDeserializeJsonToValue(
        jsonByteCount = fromJson.toByteArray(charset = Charsets.UTF_8).size.toULong(),
        typeName = "MnemonicWithPassphrase",
        serdeMessage = it.message.orEmpty()
    )
}.getOrThrow()

fun MnemonicWithPassphrase.toJson(): String = Json.encodeToString(toAndroid())

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

fun MnemonicWithPassphrase.derivePublicKeys(
    paths: List<DerivationPath>,
    factorSourceId: FactorSourceIdFromHash
): List<HierarchicalDeterministicFactorInstance> {
    return derivePublicKeys(paths).map {
        HierarchicalDeterministicFactorInstance(factorSourceId, it)
    }
}

fun MnemonicWithPassphrase.sign(
    hash: Hash,
    path: DerivationPath
): SignatureWithPublicKey = mnemonicWithPassphraseSign(
    mnemonicWithPassphrase = this,
    derivationPath = path,
    hashToSign = hash
)

fun MnemonicWithPassphrase.sign(
    input: PerFactorSourceInput<out Signable.Payload, out Signable.ID>,
) = input.perTransaction.map { perTransaction ->
    perTransaction.ownedFactorInstances.map { perFactorInstance ->
        val signatureWithPublicKey = sign(
            hash = perTransaction.payload.getSignable().hash(),
            path = perFactorInstance.factorInstance.publicKey.derivationPath
        )

        HdSignature(
            input = HdSignatureInput(
                payloadId = perTransaction.payload.getSignable().getId(),
                ownedFactorInstance = perFactorInstance
            ),
            signature = signatureWithPublicKey
        )
    }
}.flatten()

fun MnemonicWithPassphrase.factorSourceId(kind: FactorSourceKind): FactorSourceIdFromHash
    = newFactorSourceIdFromHashFromMnemonicWithPassphrase(
        factorSourceKind = kind,
        mnemonicWithPassphrase = this
    )

/**
 * Class needed for compatibility for Android Wallet version 1.*.
 *
 * Android and iOS use different schema and since Sargon Rust follows the iOS schema, the android
 * counterpart hides Sargon's implementation until the wallets migrate to version 2.*.
 */
@KoverIgnore
@Serializable
private data class AndroidMnemonicWithPassphrase(
    @SerialName("mnemonic")
    val phrase: String,
    @SerialName("bip39Passphrase")
    val passphrase: String
) {

    fun toMnemonicWithPassphrase() = MnemonicWithPassphrase(
        mnemonic = Mnemonic.init(phrase = phrase),
        passphrase = passphrase
    )

    companion object {
        fun MnemonicWithPassphrase.toAndroid() = AndroidMnemonicWithPassphrase(
            phrase = mnemonic.phrase,
            passphrase = passphrase
        )
    }
}