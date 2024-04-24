package com.radixdlt.sargon.samples

import com.radixdlt.sargon.LedgerHardwareWalletModel
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newLedgerHwWalletModelSample
import com.radixdlt.sargon.newLedgerHwWalletModelSampleOther

@UsesSampleValues
val LedgerHardwareWalletModel.Companion.sample: Sample<LedgerHardwareWalletModel>
    get() = object : Sample<LedgerHardwareWalletModel> {
        override fun invoke(): LedgerHardwareWalletModel = newLedgerHwWalletModelSample()

        override fun other(): LedgerHardwareWalletModel = newLedgerHwWalletModelSampleOther()

    }