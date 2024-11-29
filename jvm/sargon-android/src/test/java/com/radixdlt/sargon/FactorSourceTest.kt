package com.radixdlt.sargon

import android.bluetooth.BluetoothClass.Device
import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.babylon
import com.radixdlt.sargon.extensions.id
import com.radixdlt.sargon.extensions.isMain
import com.radixdlt.sargon.extensions.kind
import com.radixdlt.sargon.extensions.olympia
import com.radixdlt.sargon.extensions.supportsBabylon
import com.radixdlt.sargon.extensions.supportsOlympia
import com.radixdlt.sargon.extensions.name
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleRandom
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class FactorSourceTest : SampleTestable<FactorSource> {

    override val samples: List<Sample<FactorSource>>
        get() = listOf(FactorSource.sample)

    @Test
    fun testKind() {
        assertEquals(
            FactorSourceKind.DEVICE,
            FactorSource.sample().kind
        )

        assertEquals(
            FactorSourceKind.LEDGER_HQ_HARDWARE_WALLET,
            FactorSource.sample.other().kind
        )

        assertEquals(
            FactorSourceKind.TRUSTED_CONTACT,
            trustedContact.kind
        )

        assertEquals(
            FactorSourceKind.ARCULUS_CARD,
            arculusCard.kind
        )

        assertEquals(
            FactorSourceKind.OFF_DEVICE_MNEMONIC,
            offDeviceMnemonic.kind
        )

        assertEquals(
            FactorSourceKind.SECURITY_QUESTIONS,
            sequrityQuestions.kind
        )
    }

    @Test
    fun testId() {
        with(FactorSource.sample()) {
            assertEquals(
                id,
                (this as FactorSource.Device).value.id.asGeneral()
            )
        }

        with(FactorSource.sample.other()) {
            assertEquals(
                id,
                (this as FactorSource.Ledger).value.id.asGeneral()
            )
        }


        with(trustedContact) {
            assertEquals(
                id,
                value.id.asGeneral()
            )
        }

        with(arculusCard) {
            assertEquals(
                id,
                value.id.asGeneral()
            )
        }

        with(offDeviceMnemonic) {
            assertEquals(
                id,
                value.id.asGeneral()
            )
        }

        with(sequrityQuestions) {
            assertEquals(
                id,
                value.id.asGeneral()
            )
        }
    }

    @Test
    fun testValuesAsGeneral() {
        assertEquals(
            FactorSource.sample(),
            (FactorSource.sample() as FactorSource.Device).value.asGeneral()
        )

        assertEquals(
            FactorSource.sample.other(),
            (FactorSource.sample.other() as FactorSource.Ledger).value.asGeneral()
        )

        assertEquals(
            trustedContact,
            trustedContact.value.asGeneral()
        )

        assertEquals(
            arculusCard,
            arculusCard.value.asGeneral()
        )

        assertEquals(
            offDeviceMnemonic,
            offDeviceMnemonic.value.asGeneral()
        )

        assertEquals(
            sequrityQuestions,
            sequrityQuestions.value.asGeneral()
        )
    }

    @Test
    fun testName() {
        assertEquals(
            "My Phone",
            FactorSource.sample().name
        )
    }

    @Test
    fun testNewBabylonIsMain() {
        assertTrue(
            FactorSource.Device.babylon(
                isMain = true,
                mnemonicWithPassphrase = MnemonicWithPassphrase.sample(),
                hostInfo = HostInfo.sample()
            ).isMain
        )
    }

    @Test
    fun testNewBabylonIsNotMain() {
        assertFalse(
            FactorSource.Device.babylon(
                isMain = false,
                mnemonicWithPassphrase = MnemonicWithPassphrase.sample(),
                hostInfo = HostInfo.sample()
            ).isMain
        )
    }

    @Test
    fun testNewBabylon() {
        val factorSource = FactorSource.Device.babylon(
            isMain = false,
            mnemonicWithPassphrase = MnemonicWithPassphrase.sample(),
            hostInfo = HostInfo.sample()
        )
        assertTrue(factorSource.supportsBabylon)
        assertFalse(factorSource.supportsOlympia)
    }

    @Test
    fun testNewOlympia() {
        val factorSource = FactorSource.Device.olympia(
            mnemonicWithPassphrase = MnemonicWithPassphrase.sample(),
            hostInfo = HostInfo.sample()
        )
        assertTrue(factorSource.supportsOlympia)
        assertFalse(factorSource.supportsBabylon)
    }

    private val trustedContact = FactorSource.TrustedContact(
        value = TrustedContactFactorSource(
            id = FactorSourceIdFromAddress(
                kind = FactorSourceKind.TRUSTED_CONTACT, body = AccountAddress.sampleMainnet()
            ),
            common = FactorSourceCommon(
                cryptoParameters = FactorSourceCryptoParameters(
                    supportedCurves = listOf(Slip10Curve.CURVE25519),
                    supportedDerivationPathSchemes = listOf(DerivationPathScheme.CAP26)
                ),
                addedOn = Timestamp.now(),
                lastUsedOn = Timestamp.now(),
                flags = emptyList()
            ),
            contact = TrustedContactFactorSourceContact(
                emailAddress = EmailAddress("mail@email.com"),
                name = DisplayName("Trusted contact")
            )
        )
    )

    private val arculusCard = FactorSource.ArculusCard(
        value = ArculusCardFactorSource(
            id = FactorSourceIdFromHash(
                kind = FactorSourceKind.ARCULUS_CARD,
                body = Exactly32Bytes.sample()
            ),
            common = FactorSourceCommon(
                cryptoParameters = FactorSourceCryptoParameters(
                    supportedCurves = listOf(Slip10Curve.CURVE25519),
                    supportedDerivationPathSchemes = listOf(DerivationPathScheme.CAP26)
                ),
                addedOn = Timestamp.now(),
                lastUsedOn = Timestamp.now(),
                flags = emptyList()
            ),
            hint = ArculusCardHint(
                name = "My Arculus",
                model = ArculusCardModel.ARCULUS_COLD_STORAGE_WALLET
            )
        )
    )

    private val offDeviceMnemonic = FactorSource.OffDeviceMnemonic(
        value = OffDeviceMnemonicFactorSource(
            id = FactorSourceIdFromHash(
                kind = FactorSourceKind.ARCULUS_CARD,
                body = Exactly32Bytes.sample()
            ),
            common = FactorSourceCommon(
                cryptoParameters = FactorSourceCryptoParameters(
                    supportedCurves = listOf(Slip10Curve.CURVE25519),
                    supportedDerivationPathSchemes = listOf(DerivationPathScheme.CAP26)
                ),
                addedOn = Timestamp.now(),
                lastUsedOn = Timestamp.now(),
                flags = emptyList()
            ),
            hint = OffDeviceMnemonicHint(
                displayName = DisplayName("My mnemonic stored somewhere")
            )
        )
    )

    private val sequrityQuestions = FactorSource.SecurityQuestions(
        value = SecurityQuestionsNotProductionReadyFactorSource(
            id = FactorSourceIdFromHash(
                kind = FactorSourceKind.SECURITY_QUESTIONS,
                body = Exactly32Bytes.sample()
            ),
            common = FactorSourceCommon(
                cryptoParameters = FactorSourceCryptoParameters(
                    supportedCurves = listOf(Slip10Curve.CURVE25519),
                    supportedDerivationPathSchemes = listOf(DerivationPathScheme.CAP26)
                ),
                addedOn = Timestamp.now(),
                lastUsedOn = Timestamp.now(),
                flags = emptyList()
            ),
            sealedMnemonic = SecurityQuestionsSealedNotProductionReadyMnemonic(
                securityQuestions = emptyList(),
                kdfScheme = SecurityQuestionsNotProductionReadyKdfScheme.Version1(
                    v1 = SecurityQuestionsNotProductionReadyKdfSchemeVersion1(
                        kdfEncryptionKeysFromKeyExchangeKeys = SecurityQuestionsNotProductionReadyEncryptionKeysByDiffieHellmanFold(),
                        kdfKeyExchangesKeysFromQuestionsAndAnswers = SecurityQuestionsNotProductionReadyKeyExchangeKeysFromQandAsLowerTrimUtf8()
                    )
                ),
                encryptionScheme = EncryptionScheme.Version1(
                    v1 = AesGcm256()
                ),
                encryptions = emptyList()
            ),
        )
    )
}