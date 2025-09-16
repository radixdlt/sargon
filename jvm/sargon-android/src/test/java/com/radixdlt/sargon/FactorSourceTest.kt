package com.radixdlt.sargon

import android.bluetooth.BluetoothClass.Device
import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.babylon
import com.radixdlt.sargon.extensions.id
import com.radixdlt.sargon.extensions.kind
import com.radixdlt.sargon.extensions.olympia
import com.radixdlt.sargon.extensions.supportsBabylon
import com.radixdlt.sargon.extensions.supportsOlympia
import com.radixdlt.sargon.extensions.name
import com.radixdlt.sargon.extensions.string
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

//        assertEquals(
//            FactorSourceKind.TRUSTED_CONTACT,
//            trustedContact.kind
//        )

        assertEquals(
            FactorSourceKind.ARCULUS_CARD,
            arculusCard.kind
        )

        assertEquals(
            FactorSourceKind.OFF_DEVICE_MNEMONIC,
            offDeviceMnemonic.kind
        )

//        assertEquals(
//            FactorSourceKind.SECURITY_QUESTIONS,
//            sequrityQuestions.kind
//        )

        assertEquals(
            FactorSourceKind.PASSWORD,
            password.kind
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


//        with(trustedContact) {
//            assertEquals(
//                id,
//                value.id.asGeneral()
//            )
//        }

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

//        with(sequrityQuestions) {
//            assertEquals(
//                id,
//                value.id.asGeneral()
//            )
//        }

        with(password) {
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

//        assertEquals(
//            trustedContact,
//            trustedContact.value.asGeneral()
//        )

        assertEquals(
            arculusCard,
            arculusCard.value.asGeneral()
        )

        assertEquals(
            offDeviceMnemonic,
            offDeviceMnemonic.value.asGeneral()
        )

//        assertEquals(
//            sequrityQuestions,
//            sequrityQuestions.value.asGeneral()
//        )

        assertEquals(
            password,
            password.value.asGeneral()
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
    fun testNewBabylon() {
        val factorSource = FactorSource.Device.babylon(
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

//    private val trustedContact = TrustedContactFactorSource.sample().asGeneral()

    private val arculusCard = ArculusCardFactorSource.sample().asGeneral()

    private val offDeviceMnemonic = OffDeviceMnemonicFactorSource.sample().asGeneral()

//    private val sequrityQuestions = SecurityQuestionsNotProductionReadyFactorSource.sample().asGeneral()

    private val password = PasswordFactorSource.sample().asGeneral()
}
