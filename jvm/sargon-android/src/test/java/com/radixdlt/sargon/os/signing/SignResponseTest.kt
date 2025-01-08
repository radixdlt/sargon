package com.radixdlt.sargon.os.signing

import com.radixdlt.sargon.Account
import com.radixdlt.sargon.AddressOfAccountOrPersona
import com.radixdlt.sargon.AuthIntent
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.EntitySecurityState
import com.radixdlt.sargon.FactorOutcomeOfAuthIntentHash
import com.radixdlt.sargon.FactorOutcomeOfSubintentHash
import com.radixdlt.sargon.FactorOutcomeOfTransactionIntentHash
import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.HdSignatureInputOfAuthIntentHash
import com.radixdlt.sargon.HdSignatureInputOfSubintentHash
import com.radixdlt.sargon.HdSignatureInputOfTransactionIntentHash
import com.radixdlt.sargon.HdSignatureOfAuthIntentHash
import com.radixdlt.sargon.HdSignatureOfSubintentHash
import com.radixdlt.sargon.HdSignatureOfTransactionIntentHash
import com.radixdlt.sargon.HierarchicalDeterministicFactorInstance
import com.radixdlt.sargon.HierarchicalDeterministicPublicKey
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.NeglectFactorReason
import com.radixdlt.sargon.NeglectedFactor
import com.radixdlt.sargon.OwnedFactorInstance
import com.radixdlt.sargon.PerFactorOutcomeOfAuthIntentHash
import com.radixdlt.sargon.PerFactorOutcomeOfSubintentHash
import com.radixdlt.sargon.PerFactorOutcomeOfTransactionIntentHash
import com.radixdlt.sargon.SignResponseOfAuthIntentHash
import com.radixdlt.sargon.SignResponseOfSubintentHash
import com.radixdlt.sargon.SignResponseOfTransactionIntentHash
import com.radixdlt.sargon.Subintent
import com.radixdlt.sargon.TransactionIntent
import com.radixdlt.sargon.extensions.hash
import com.radixdlt.sargon.extensions.publicKey
import com.radixdlt.sargon.extensions.sign
import com.radixdlt.sargon.extensions.signed
import com.radixdlt.sargon.extensions.skipped
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class SignResponseTest {

    private val mnemonic = MnemonicWithPassphrase.sample()
    private val account = Account.sampleMainnet()
    private val ownedFactorInstance = OwnedFactorInstance(
        owner = AddressOfAccountOrPersona.Account(account.address),
        factorInstance = (account.securityState as EntitySecurityState.Unsecured)
            .value
            .transactionSigning
    )

    @Test
    fun testSignResponseForTransactionIntentHashSigned() {
        val factorSourceId = ownedFactorInstance.factorInstance.factorSourceId
        val payload = TransactionIntent.sample()
        val signature = mnemonic.sign(
            hash = payload.hash().hash,
            path = ownedFactorInstance.factorInstance.publicKey.derivationPath
        )

        val signResponse = SignResponse(
            perFactorOutcome = listOf(
                PerFactorOutcome(
                    factorSourceId = factorSourceId,
                    outcome = FactorOutcome.Signed(
                        producedSignatures = listOf(
                            HdSignature(
                                input = HdSignatureInput(
                                    payloadId = Signable.ID.Transaction(payload.hash()),
                                    ownedFactorInstance = ownedFactorInstance
                                ),
                                signature = signature
                            )
                        )
                    )
                ),
                PerFactorOutcome(
                    factorSourceId = FactorSourceIdFromHash.sample.other(),
                    outcome = FactorOutcome.Neglected(
                        factor = NeglectedFactor(
                            reason = NeglectFactorReason.USER_EXPLICITLY_SKIPPED,
                            factor = FactorSourceIdFromHash.sample.other()
                        )
                    )
                )
            )
        )

        assertEquals(
            SignResponseOfTransactionIntentHash(
                perFactorOutcome = listOf(
                    PerFactorOutcomeOfTransactionIntentHash(
                        factorSourceId = FactorSourceIdFromHash.sample(),
                        outcome = FactorOutcomeOfTransactionIntentHash.signed(
                            producedSignatures = listOf(
                                HdSignatureOfTransactionIntentHash(
                                    input = HdSignatureInputOfTransactionIntentHash(
                                        payloadId = payload.hash(),
                                        ownedFactorInstance = ownedFactorInstance
                                    ),
                                    signature = signature
                                )
                            )
                        )
                    ),
                    PerFactorOutcomeOfTransactionIntentHash(
                        factorSourceId = FactorSourceIdFromHash.sample.other(),
                        outcome = FactorOutcomeOfTransactionIntentHash.skipped(
                            FactorSourceIdFromHash.sample.other()
                        )
                    )
                )
            ),
            signResponse.intoSargon(),
        )
    }

    @Test
    fun testSignResponseForSubintentHashSigned() {
        val factorSourceId = ownedFactorInstance.factorInstance.factorSourceId
        val payload = Subintent.sample()
        val signature = mnemonic.sign(
            hash = payload.hash().hash,
            path = ownedFactorInstance.factorInstance.publicKey.derivationPath
        )

        val signResponse = SignResponse(
            perFactorOutcome = listOf(
                PerFactorOutcome(
                    factorSourceId = factorSourceId,
                    outcome = FactorOutcome.Signed(
                        producedSignatures = listOf(
                            HdSignature(
                                input = HdSignatureInput(
                                    payloadId = Signable.ID.Subintent(payload.hash()),
                                    ownedFactorInstance = ownedFactorInstance
                                ),
                                signature = signature
                            )
                        )
                    )
                ),
                PerFactorOutcome(
                    factorSourceId = FactorSourceIdFromHash.sample.other(),
                    outcome = FactorOutcome.Neglected(
                        factor = NeglectedFactor(
                            reason = NeglectFactorReason.USER_EXPLICITLY_SKIPPED,
                            factor = FactorSourceIdFromHash.sample.other()
                        )
                    )
                )
            )
        )

        assertEquals(
            SignResponseOfSubintentHash(
                perFactorOutcome = listOf(
                    PerFactorOutcomeOfSubintentHash(
                        factorSourceId = FactorSourceIdFromHash.sample(),
                        outcome = FactorOutcomeOfSubintentHash.signed(
                            producedSignatures = listOf(
                                HdSignatureOfSubintentHash(
                                    input = HdSignatureInputOfSubintentHash(
                                        payloadId = payload.hash(),
                                        ownedFactorInstance = ownedFactorInstance
                                    ),
                                    signature = signature
                                )
                            )
                        )
                    ),
                    PerFactorOutcomeOfSubintentHash(
                        factorSourceId = FactorSourceIdFromHash.sample.other(),
                        outcome = FactorOutcomeOfSubintentHash.skipped(
                            FactorSourceIdFromHash.sample.other()
                        )
                    )
                )
            ),
            signResponse.intoSargon(),
        )
    }

    @Test
    fun testSignResponseForAuthIntentHashSigned() {
        val factorSourceId = ownedFactorInstance.factorInstance.factorSourceId
        val payload = AuthIntent.sample()
        val signature = mnemonic.sign(
            hash = payload.hash().payload.hash(),
            path = ownedFactorInstance.factorInstance.publicKey.derivationPath
        )

        val signResponse = SignResponse(
            perFactorOutcome = listOf(
                PerFactorOutcome(
                    factorSourceId = factorSourceId,
                    outcome = FactorOutcome.Signed(
                        producedSignatures = listOf(
                            HdSignature(
                                input = HdSignatureInput(
                                    payloadId = Signable.ID.Auth(payload.hash()),
                                    ownedFactorInstance = ownedFactorInstance
                                ),
                                signature = signature
                            )
                        )
                    )
                ),
                PerFactorOutcome(
                    factorSourceId = FactorSourceIdFromHash.sample.other(),
                    outcome = FactorOutcome.Neglected(
                        factor = NeglectedFactor(
                            reason = NeglectFactorReason.USER_EXPLICITLY_SKIPPED,
                            factor = FactorSourceIdFromHash.sample.other()
                        )
                    )
                )
            )
        )

        assertEquals(
            SignResponseOfAuthIntentHash(
                perFactorOutcome = listOf(
                    PerFactorOutcomeOfAuthIntentHash(
                        factorSourceId = FactorSourceIdFromHash.sample(),
                        outcome = FactorOutcomeOfAuthIntentHash.signed(
                            producedSignatures = listOf(
                                HdSignatureOfAuthIntentHash(
                                    input = HdSignatureInputOfAuthIntentHash(
                                        payloadId = payload.hash(),
                                        ownedFactorInstance = ownedFactorInstance
                                    ),
                                    signature = signature
                                )
                            )
                        )
                    ),
                    PerFactorOutcomeOfAuthIntentHash(
                        factorSourceId = FactorSourceIdFromHash.sample.other(),
                        outcome = FactorOutcomeOfAuthIntentHash.skipped(
                            FactorSourceIdFromHash.sample.other()
                        )
                    )
                )
            ),
            signResponse.intoSargon(),
        )
    }

    @Test
    fun testSignResponseForTransactionIntentHashInvalidOutcomesWhenSigned() {
        val payload = TransactionIntent.sample()
        val signature = mnemonic.sign(
            hash = payload.hash().hash,
            path = ownedFactorInstance.factorInstance.publicKey.derivationPath
        )

        val signResponse = SignResponse(
            perFactorOutcome = listOf(
                PerFactorOutcome(
                    factorSourceId = FactorSourceIdFromHash.sample(),
                    outcome = FactorOutcome.Signed(
                        producedSignatures = listOf(
                            HdSignature(
                                input = HdSignatureInput(
                                    payloadId = Signable.ID.Transaction(payload.hash()),
                                    ownedFactorInstance = OwnedFactorInstance(
                                        owner = ownedFactorInstance.owner,
                                        factorInstance = HierarchicalDeterministicFactorInstance(
                                            // Using a different factor source id
                                            factorSourceId = FactorSourceIdFromHash.sample.other(),
                                            publicKey = ownedFactorInstance.factorInstance.publicKey
                                        )
                                    )
                                ),
                                signature = signature
                            )
                        )
                    )
                ),
            )
        )

        val error = runCatching {
            signResponse.intoSargon()
        }.exceptionOrNull()

        assertTrue(error is CommonException.FactorOutcomeSignedFactorSourceIdMismatch)
    }

    @Test
    fun testSignResponseForTransactionIntentHashInvalidOutcomesWhenNeglected() {
        val signResponse = SignResponse<Signable.ID.Transaction>(
            perFactorOutcome = listOf(
                PerFactorOutcome(
                    // Using a different factor source id
                    factorSourceId = FactorSourceIdFromHash.sample.other(),
                    outcome = FactorOutcome.Neglected(
                        factor = NeglectedFactor(
                            reason = NeglectFactorReason.USER_EXPLICITLY_SKIPPED,
                            factor = FactorSourceIdFromHash.sample()
                        )
                    )
                ),
            )
        )

        val error = runCatching {
            signResponse.intoSargon()
        }.exceptionOrNull()

        assertTrue(error is CommonException.FactorOutcomeSignedFactorSourceIdMismatch)
    }

    @Test
    fun testSignResponseForSubintentHashInvalidOutcomesWhenSigned() {
        val payload = Subintent.sample()
        val signature = mnemonic.sign(
            hash = payload.hash().hash,
            path = ownedFactorInstance.factorInstance.publicKey.derivationPath
        )

        val signResponse = SignResponse(
            perFactorOutcome = listOf(
                PerFactorOutcome(
                    factorSourceId = FactorSourceIdFromHash.sample(),
                    outcome = FactorOutcome.Signed(
                        producedSignatures = listOf(
                            HdSignature(
                                input = HdSignatureInput(
                                    payloadId = Signable.ID.Subintent(payload.hash()),
                                    ownedFactorInstance = OwnedFactorInstance(
                                        owner = ownedFactorInstance.owner,
                                        factorInstance = HierarchicalDeterministicFactorInstance(
                                            // Using a different factor source id
                                            factorSourceId = FactorSourceIdFromHash.sample.other(),
                                            publicKey = ownedFactorInstance.factorInstance.publicKey
                                        )
                                    )
                                ),
                                signature = signature
                            )
                        )
                    )
                ),
            )
        )

        val error = runCatching {
            signResponse.intoSargon()
        }.exceptionOrNull()

        assertTrue(error is CommonException.FactorOutcomeSignedFactorSourceIdMismatch)
    }

    @Test
    fun testSignResponseForSubintentHashInvalidOutcomesWhenNeglected() {
        val signResponse = SignResponse<Signable.ID.Subintent>(
            perFactorOutcome = listOf(
                PerFactorOutcome(
                    // Using a different factor source id
                    factorSourceId = FactorSourceIdFromHash.sample.other(),
                    outcome = FactorOutcome.Neglected(
                        factor = NeglectedFactor(
                            reason = NeglectFactorReason.USER_EXPLICITLY_SKIPPED,
                            factor = FactorSourceIdFromHash.sample()
                        )
                    )
                ),
            )
        )

        val error = runCatching {
            signResponse.intoSargon()
        }.exceptionOrNull()

        assertTrue(error is CommonException.FactorOutcomeSignedFactorSourceIdMismatch)
    }

    @Test
    fun testSignResponseForAuthIntentHashInvalidOutcomesWhenSigned() {
        val payload = AuthIntent.sample()
        val signature = mnemonic.sign(
            hash = payload.hash().payload.hash(),
            path = ownedFactorInstance.factorInstance.publicKey.derivationPath
        )

        val signResponse = SignResponse(
            perFactorOutcome = listOf(
                PerFactorOutcome(
                    factorSourceId = FactorSourceIdFromHash.sample(),
                    outcome = FactorOutcome.Signed(
                        producedSignatures = listOf(
                            HdSignature(
                                input = HdSignatureInput(
                                    payloadId = Signable.ID.Auth(payload.hash()),
                                    ownedFactorInstance = OwnedFactorInstance(
                                        owner = ownedFactorInstance.owner,
                                        factorInstance = HierarchicalDeterministicFactorInstance(
                                            // Using a different factor source id
                                            factorSourceId = FactorSourceIdFromHash.sample.other(),
                                            publicKey = ownedFactorInstance.factorInstance.publicKey
                                        )
                                    )
                                ),
                                signature = signature
                            )
                        )
                    )
                ),
            )
        )

        val error = runCatching {
            signResponse.intoSargon()
        }.exceptionOrNull()

        assertTrue(error is CommonException.FactorOutcomeSignedFactorSourceIdMismatch)
    }

    @Test
    fun testSignResponseForAuthIntentHashInvalidOutcomesWhenNeglected() {
        val signResponse = SignResponse<Signable.ID.Auth>(
            perFactorOutcome = listOf(
                PerFactorOutcome(
                    // Using a different factor source id
                    factorSourceId = FactorSourceIdFromHash.sample.other(),
                    outcome = FactorOutcome.Neglected(
                        factor = NeglectedFactor(
                            reason = NeglectFactorReason.USER_EXPLICITLY_SKIPPED,
                            factor = FactorSourceIdFromHash.sample()
                        )
                    )
                ),
            )
        )

        val error = runCatching {
            signResponse.intoSargon()
        }.exceptionOrNull()

        assertTrue(error is CommonException.FactorOutcomeSignedFactorSourceIdMismatch)
    }
}