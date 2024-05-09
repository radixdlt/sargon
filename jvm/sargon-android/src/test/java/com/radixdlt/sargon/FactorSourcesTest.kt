package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.FactorSources
import com.radixdlt.sargon.extensions.SupportedCurves
import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.id
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

internal class FactorSourcesTest : IdentifiedArrayTest<FactorSources, FactorSourceId, FactorSource>() {

    override fun element(): FactorSource = FactorSource.sample()

    override fun elementWithDifferentId(): FactorSource = FactorSource.sample.other()

    override fun identifier(element: FactorSource): FactorSourceId = element.id

    override fun init(element: FactorSource): FactorSources = FactorSources(element)

    @Test
    fun testDeviceFactorSourceAsGeneral() {
        val factorSource = DeviceFactorSource(
            id = FactorSourceIdFromHash(
                kind = FactorSourceKind.DEVICE,
                body = Exactly32Bytes.init(randomBagOfBytes(32))
            ),
            common = FactorSourceCommon(
                cryptoParameters = FactorSourceCryptoParameters(
                    supportedCurves = SupportedCurves(Slip10Curve.CURVE25519).asList(),
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
                    supportedCurves = SupportedCurves(Slip10Curve.CURVE25519).asList(),
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