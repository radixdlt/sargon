package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.LedgerHardwareWalletFactorSource
import com.radixdlt.sargon.newLedgerHardwareWalletFactorSourceSample
import com.radixdlt.sargon.newLedgerHardwareWalletFactorSourceSampleOther

@UsesSampleValues
val LedgerHardwareWalletFactorSource.Companion.sample: Sample<LedgerHardwareWalletFactorSource>
    get() = object : Sample<LedgerHardwareWalletFactorSource> {

        override fun invoke(): LedgerHardwareWalletFactorSource = newLedgerHardwareWalletFactorSourceSample()

        override fun other(): LedgerHardwareWalletFactorSource = newLedgerHardwareWalletFactorSourceSampleOther()
    }