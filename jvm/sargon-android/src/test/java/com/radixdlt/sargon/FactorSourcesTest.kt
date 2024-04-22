package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.append
import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.contains
import com.radixdlt.sargon.extensions.get
import com.radixdlt.sargon.extensions.id
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.invoke
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.remove
import com.radixdlt.sargon.extensions.removeById
import com.radixdlt.sargon.extensions.size
import com.radixdlt.sargon.extensions.updateOrAppend
import com.radixdlt.sargon.extensions.updateOrInsert
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class FactorSourcesTest : SampleTestable<FactorSources> {

    override val samples: List<Sample<FactorSources>>
        get() = listOf(FactorSources.sample)

    @Test
    fun testListMethods() {
        val sample = FactorSource.sample()
        assertEquals(sample.id, FactorSource.sample().id)
        val sampleOther = FactorSource.sample.other()

        var list = FactorSources.init(sample)

        Assertions.assertTrue(sample in list)
        assertEquals(1, list.size)
        assertEquals(sample, list[0])

        list = list.append(sampleOther)
        Assertions.assertTrue(sampleOther in list)
        assertEquals(2, list.size)
        assertEquals(sampleOther, list[1])

        list = list.remove(sampleOther)
        Assertions.assertFalse(sampleOther in list)
        assertEquals(1, list.size)

        list = list.append(sampleOther)
        assertEquals(sampleOther, list.get(sampleOther.id))
        list = list.removeById(sampleOther.id)
        Assertions.assertTrue(list.size == 1)

        list = list.updateOrInsert(sampleOther, 0)
        assertEquals(sampleOther, list()[0])
        Assertions.assertTrue(list.size == 2)
        list = list.updateOrAppend(sampleOther)
        Assertions.assertTrue(list.size == 2)
    }

    @Test
    fun testEmptyFactorSourcesFails() {
        assertThrows<CommonException.FactorSourcesMustNotBeEmpty> {
            FactorSources.init()
        }
    }

    @Test
    fun testDeviceFactorSourceAsGeneral() {
        val factorSource = DeviceFactorSource(
            id = FactorSourceIdFromHash(
                kind = FactorSourceKind.DEVICE,
                body = Exactly32Bytes.init(randomBagOfBytes(32))
            ),
            common = FactorSourceCommon(
                cryptoParameters = FactorSourceCryptoParameters(
                    supportedCurves = SupportedCurves.init(Slip10Curve.CURVE25519),
                    supportedDerivationPathSchemes = listOf(DerivationPathScheme.CAP26)
                ),
                addedOn = Timestamp.now(),
                lastUsedOn = Timestamp.now(),
                flags = emptyList()
            ),
            hint = DeviceFactorSourceHint(
                name = "Unit",
                model = "Test",
                mnemonicWordCount = Bip39WordCount.TWENTY_FOUR
            )
        )

        assertEquals(
            FactorSource.Device(factorSource),
            factorSource.asGeneral()
        )
    }

    @Test
    fun testLedgerFactorSourceAsGeneral() {
        val factorSource = LedgerHardwareWalletFactorSource(
            id = FactorSourceIdFromHash(
                kind = FactorSourceKind.DEVICE,
                body = Exactly32Bytes.init(randomBagOfBytes(32))
            ),
            common = FactorSourceCommon(
                cryptoParameters = FactorSourceCryptoParameters(
                    supportedCurves = SupportedCurves.init(Slip10Curve.CURVE25519),
                    supportedDerivationPathSchemes = listOf(DerivationPathScheme.CAP26)
                ),
                addedOn = Timestamp.now(),
                lastUsedOn = Timestamp.now(),
                flags = emptyList()
            ),
            hint = LedgerHardwareWalletHint(
                name = "Unit",
                model = LedgerHardwareWalletModel.NANO_S
            )
        )

        assertEquals(
            FactorSource.Ledger(factorSource),
            factorSource.asGeneral()
        )
    }

}