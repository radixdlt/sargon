package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.compile
import com.radixdlt.sargon.extensions.derivePublicKey
import com.radixdlt.sargon.extensions.derivePublicKeys
import com.radixdlt.sargon.extensions.factorSourceId
import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.getAuthSignatures
import com.radixdlt.sargon.extensions.getSubintentSignatures
import com.radixdlt.sargon.extensions.getTransactionSignatures
import com.radixdlt.sargon.extensions.hash
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.initForEntity
import com.radixdlt.sargon.extensions.initFromLocal
import com.radixdlt.sargon.extensions.isValidSignature
import com.radixdlt.sargon.extensions.phrase
import com.radixdlt.sargon.extensions.sign
import com.radixdlt.sargon.extensions.signature
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.extensions.validate
import com.radixdlt.sargon.os.signing.HdSignature
import com.radixdlt.sargon.os.signing.HdSignatureInput
import com.radixdlt.sargon.os.signing.PerFactorSourceInput
import com.radixdlt.sargon.os.signing.Signable
import com.radixdlt.sargon.os.signing.TransactionSignRequestInput
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class MnemonicWithPassphraseTest {

    @Test
    fun testJsonRoundtrip() {
        val mnemonicWithPassphrase = MnemonicWithPassphrase(
            mnemonic = Mnemonic.init("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"),
            passphrase = "super secret"
        )

        assertEquals(
            mnemonicWithPassphrase,
            MnemonicWithPassphrase.fromJson(mnemonicWithPassphrase.toJson())
        )
    }

    @Test
    fun testInitFromJustPhrase() {
        val phrase = "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
        assertEquals(phrase, MnemonicWithPassphrase.init(phrase = phrase).mnemonic.phrase)
    }

    @Test
    fun testFromAndroidJson() {
        val androidJsonWithoutPassphrase =
            """{"mnemonic":"remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track","bip39Passphrase":""}""".trimIndent()
        assertEquals(
            MnemonicWithPassphrase(
                mnemonic = Mnemonic.init(phrase = "remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track"),
                passphrase = ""
            ),
            MnemonicWithPassphrase.fromJson(androidJsonWithoutPassphrase)
        )

        val androidJsonWithoutPassphrasePrettyPrinted = """{
              "mnemonic": "remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track",
              "bip39Passphrase": ""
            }""".trimIndent()
        assertEquals(
            MnemonicWithPassphrase(
                mnemonic = Mnemonic.init(phrase = "remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track"),
                passphrase = ""
            ),
            MnemonicWithPassphrase.fromJson(androidJsonWithoutPassphrasePrettyPrinted)
        )

        val androidJsonWithPassphrase =
            """{"mnemonic":"remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track","bip39Passphrase":"super secret"}""".trimIndent()
        assertEquals(
            MnemonicWithPassphrase(
                mnemonic = Mnemonic.init(phrase = "remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track"),
                passphrase = "super secret"
            ),
            MnemonicWithPassphrase.fromJson(androidJsonWithPassphrase)
        )
    }

    @Test
    fun testFromInvalidJson() {
        val invalidJson = "{}"
        assertThrows<CommonException.FailedToDeserializeJsonToValue> {
            MnemonicWithPassphrase.fromJson(invalidJson)
        }

        val iOSJsonLike = mnemonicWithPassphraseToJsonBytes(
            MnemonicWithPassphrase(
                mnemonic = Mnemonic.init("remind index lift gun sleep inner double leopard exist sugar item whisper coast duty leopard law radar neutral odor tape finger position capital track"),
                passphrase = "super secret"
            )
        ).string

        assertThrows<CommonException.FailedToDeserializeJsonToValue> {
            MnemonicWithPassphrase.fromJson(iOSJsonLike)
        }
    }

    @Test
    fun testHDPublicKeyValidation() {
        assertTrue(
            MnemonicWithPassphrase.sample()
                .validate(listOf(HierarchicalDeterministicPublicKey.sample())),
        )
        assertFalse(
            MnemonicWithPassphrase.sample.other()
                .validate(listOf(HierarchicalDeterministicPublicKey.sample())),
        )
    }

    @Test
    fun testSignIsValid() {
        val sut = MnemonicWithPassphrase.sample()
        val derivationPath = DerivationPath.sample()
        val message = Hash.sample()

        val publicKey = sut.derivePublicKey(path = derivationPath)
        val signatureWithPublicKey = sut.sign(message, derivationPath)
        assertTrue(
            publicKey.isValidSignature(signatureWithPublicKey.signature, message)
        )
    }

    @Test
    fun testDerivePublicKeys() {
        assertEquals(
            HierarchicalDeterministicPublicKey.sample(),
            MnemonicWithPassphrase.sample().derivePublicKey(DerivationPath.sample())
        )
    }

    @Test
    fun testDerivePublicKeysFactorInstances() {
        assertEquals(
            listOf(HierarchicalDeterministicFactorInstance.sample()),
            MnemonicWithPassphrase.sample()
                .derivePublicKeys(listOf(DerivationPath.sample()), FactorSourceIdFromHash.sample())
        )
    }

    @Test
    fun testFactorSourceId() {
        val mnemonic = MnemonicWithPassphrase.sample()

        assertEquals(
            mnemonic.factorSourceId(kind = FactorSourceKind.DEVICE).kind,
            FactorSourceKind.DEVICE
        )
        assertEquals(
            mnemonic.factorSourceId(kind = FactorSourceKind.OFF_DEVICE_MNEMONIC).kind,
            FactorSourceKind.OFF_DEVICE_MNEMONIC
        )
    }

    @Test
    fun testSignSignables() {
        val mnemonic = MnemonicWithPassphrase.sample()
        val fs = mnemonic.factorSourceId(kind = FactorSourceKind.DEVICE)

        val tx1 = TransactionIntent.sample()
        val tx2 = TransactionIntent.sample.other()

        val acc1 = OwnedFactorInstance(
            owner = AddressOfAccountOrPersona.Account(
                AccountAddress.sampleMainnet()
            ),
            factorInstance = HierarchicalDeterministicFactorInstance(
                factorSourceId = fs,
                publicKey = mnemonic.derivePublicKey(
                    DerivationPath.initForEntity(
                        kind = EntityKind.ACCOUNT,
                        networkId = NetworkId.MAINNET,
                        index = Hardened.Unsecurified(UnsecurifiedHardened.initFromLocal(0u))
                    )
                )
            )
        )

        val acc2 = OwnedFactorInstance(
            owner = AddressOfAccountOrPersona.Account(
                AccountAddress.sampleMainnet.other()
            ),
            factorInstance = HierarchicalDeterministicFactorInstance(
                factorSourceId = fs,
                publicKey = mnemonic.derivePublicKey(
                    DerivationPath.initForEntity(
                        kind = EntityKind.ACCOUNT,
                        networkId = NetworkId.MAINNET,
                        index = Hardened.Unsecurified(UnsecurifiedHardened.initFromLocal(1u))
                    )
                )
            )
        )

        val input = PerFactorSourceInput<Signable.Payload.Transaction, Signable.ID.Transaction>(
            factorSourceId = fs,
            perTransaction = listOf(
                TransactionSignRequestInput(
                    payload = Signable.Payload.Transaction(tx1.compile()),
                    factorSourceId = fs,
                    ownedFactorInstances = listOf(acc1, acc2)
                ),
                TransactionSignRequestInput(
                    payload = Signable.Payload.Transaction(tx2.compile()),
                    factorSourceId = fs,
                    ownedFactorInstances = listOf(acc1, acc2)
                )
            ),
            invalidTransactionsIfNeglected = emptyList()
        )

        val hdSignatures = mnemonic.sign(input)

        assertEquals(
            listOf(
                HdSignature(
                    input = HdSignatureInput(
                        payloadId = Signable.ID.Transaction(tx1.hash()),
                        ownedFactorInstance = acc1
                    ),
                    signature = mnemonic.sign(tx1.hash().hash, acc1.factorInstance.publicKey.derivationPath)
                ),
                HdSignature(
                    input = HdSignatureInput(
                        payloadId = Signable.ID.Transaction(tx1.hash()),
                        ownedFactorInstance = acc2
                    ),
                    signature = mnemonic.sign(tx1.hash().hash, acc2.factorInstance.publicKey.derivationPath)
                ),
                HdSignature(
                    input = HdSignatureInput(
                        payloadId = Signable.ID.Transaction(tx2.hash()),
                        ownedFactorInstance = acc1
                    ),
                    signature = mnemonic.sign(tx2.hash().hash, acc1.factorInstance.publicKey.derivationPath)
                ),
                HdSignature(
                    input = HdSignatureInput(
                        payloadId = Signable.ID.Transaction(tx2.hash()),
                        ownedFactorInstance = acc2
                    ),
                    signature = mnemonic.sign(tx2.hash().hash, acc2.factorInstance.publicKey.derivationPath)
                )
            ),
            hdSignatures
        )
    }

    @Test
    fun testGetTransactionSignatures() {
        val mnemonic = MnemonicWithPassphrase.sample()
        val fs = mnemonic.factorSourceId(kind = FactorSourceKind.DEVICE)

        val tx1 = TransactionIntent.sample()
        val tx2 = TransactionIntent.sample.other()

        val acc1 = OwnedFactorInstance(
            owner = AddressOfAccountOrPersona.Account(AccountAddress.sampleMainnet()),
            factorInstance = HierarchicalDeterministicFactorInstance(
                factorSourceId = fs,
                publicKey = mnemonic.derivePublicKey(
                    DerivationPath.initForEntity(
                        kind = EntityKind.ACCOUNT,
                        networkId = NetworkId.MAINNET,
                        index = Hardened.Unsecurified(UnsecurifiedHardened.initFromLocal(0u))
                    )
                )
            )
        )
        val acc2 = OwnedFactorInstance(
            owner = AddressOfAccountOrPersona.Account(AccountAddress.sampleMainnet.other()),
            factorInstance = HierarchicalDeterministicFactorInstance(
                factorSourceId = fs,
                publicKey = mnemonic.derivePublicKey(
                    DerivationPath.initForEntity(
                        kind = EntityKind.ACCOUNT,
                        networkId = NetworkId.MAINNET,
                        index = Hardened.Unsecurified(UnsecurifiedHardened.initFromLocal(1u))
                    )
                )
            )
        )

        val input = PerFactorSourceInputOfTransactionIntent(
            factorSourceId = fs,
            perTransaction = listOf(
                TransactionSignRequestInputOfTransactionIntent(
                    payload = tx1.compile(),
                    factorSourceId = fs,
                    ownedFactorInstances = listOf(acc1, acc2)
                ),
                TransactionSignRequestInputOfTransactionIntent(
                    payload = tx2.compile(),
                    factorSourceId = fs,
                    ownedFactorInstances = listOf(acc1, acc2)
                )
            ),
            invalidTransactionsIfNeglected = emptyList()
        )

        val hdSignatures = mnemonic.getTransactionSignatures(input)

        assertEquals(
            listOf(
                HdSignatureOfTransactionIntentHash(
                    input = HdSignatureInputOfTransactionIntentHash(
                        payloadId = tx1.hash(),
                        ownedFactorInstance = acc1
                    ),
                    signature = mnemonic.sign(
                        tx1.hash().hash,
                        acc1.factorInstance.publicKey.derivationPath
                    )
                ),
                HdSignatureOfTransactionIntentHash(
                    input = HdSignatureInputOfTransactionIntentHash(
                        payloadId = tx1.hash(),
                        ownedFactorInstance = acc2
                    ),
                    signature = mnemonic.sign(
                        tx1.hash().hash,
                        acc2.factorInstance.publicKey.derivationPath
                    )
                ),
                HdSignatureOfTransactionIntentHash(
                    input = HdSignatureInputOfTransactionIntentHash(
                        payloadId = tx2.hash(),
                        ownedFactorInstance = acc1
                    ),
                    signature = mnemonic.sign(
                        tx2.hash().hash,
                        acc1.factorInstance.publicKey.derivationPath
                    )
                ),
                HdSignatureOfTransactionIntentHash(
                    input = HdSignatureInputOfTransactionIntentHash(
                        payloadId = tx2.hash(),
                        ownedFactorInstance = acc2
                    ),
                    signature = mnemonic.sign(
                        tx2.hash().hash,
                        acc2.factorInstance.publicKey.derivationPath
                    )
                )
            ),
            hdSignatures
        )
    }

    @Test
    fun testGetSubintentSignatures() {
        val mnemonic = MnemonicWithPassphrase.sample()
        val fs = mnemonic.factorSourceId(kind = FactorSourceKind.DEVICE)

        val sub = Subintent.sample()
        val acc = OwnedFactorInstance(
            owner = AddressOfAccountOrPersona.Account(AccountAddress.sampleMainnet()),
            factorInstance = HierarchicalDeterministicFactorInstance(
                factorSourceId = fs,
                publicKey = mnemonic.derivePublicKey(
                    DerivationPath.initForEntity(
                        kind = EntityKind.ACCOUNT,
                        networkId = NetworkId.MAINNET,
                        index = Hardened.Unsecurified(UnsecurifiedHardened.initFromLocal(0u))
                    )
                )
            )
        )

        val input = PerFactorSourceInputOfSubintent(
            factorSourceId = fs,
            perTransaction = listOf(
                TransactionSignRequestInputOfSubintent(
                    payload = sub.compile(),
                    factorSourceId = fs,
                    ownedFactorInstances = listOf(acc)
                )
            ),
            invalidTransactionsIfNeglected = emptyList()
        )

        val signatures = mnemonic.getSubintentSignatures(input)

        assertEquals(
            listOf(
                HdSignatureOfSubintentHash(
                    input = HdSignatureInputOfSubintentHash(
                        payloadId = sub.hash(),
                        ownedFactorInstance = acc
                    ),
                    signature = mnemonic.sign(
                        hash = sub.hash().hash,
                        path = acc.factorInstance.publicKey.derivationPath
                    )
                )
            ),
            signatures
        )
    }

    @Test
    fun testGetAuthSignatures() {
        val mnemonic = MnemonicWithPassphrase.sample()
        val fs = mnemonic.factorSourceId(kind = FactorSourceKind.DEVICE)

        val auth = AuthIntent.sample()
        val acc = OwnedFactorInstance(
            owner = AddressOfAccountOrPersona.Account(AccountAddress.sampleMainnet()),
            factorInstance = HierarchicalDeterministicFactorInstance(
                factorSourceId = fs,
                publicKey = mnemonic.derivePublicKey(
                    DerivationPath.initForEntity(
                        kind = EntityKind.ACCOUNT,
                        networkId = NetworkId.MAINNET,
                        index = Hardened.Unsecurified(UnsecurifiedHardened.initFromLocal(0u))
                    )
                )
            )
        )

        val input = PerFactorSourceInputOfAuthIntent(
            factorSourceId = fs,
            perTransaction = listOf(
                TransactionSignRequestInputOfAuthIntent(
                    payload = auth,
                    factorSourceId = fs,
                    ownedFactorInstances = listOf(acc)
                )
            ),
            invalidTransactionsIfNeglected = emptyList()
        )

        val signatures = mnemonic.getAuthSignatures(input)

        val payloadId = auth.hash()
        assertEquals(
            listOf(
                HdSignatureOfAuthIntentHash(
                    input = HdSignatureInputOfAuthIntentHash(
                        payloadId = payloadId,
                        ownedFactorInstance = acc
                    ),
                    signature = mnemonic.sign(
                        hash = payloadId.payload.hash(),
                        path = acc.factorInstance.publicKey.derivationPath
                    )
                )
            ),
            signatures
        )
    }
}