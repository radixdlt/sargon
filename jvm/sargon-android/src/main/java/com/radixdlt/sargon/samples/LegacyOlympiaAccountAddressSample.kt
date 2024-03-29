package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.LegacyOlympiaAccountAddress
import com.radixdlt.sargon.newLegacyOlympiaAccountAddressSample
import com.radixdlt.sargon.newLegacyOlympiaAccountAddressSampleOther

@VisibleForTesting
val LegacyOlympiaAccountAddress.Companion.sample: Sample<LegacyOlympiaAccountAddress>
    get() = object : Sample<LegacyOlympiaAccountAddress> {
        override fun invoke(): LegacyOlympiaAccountAddress = newLegacyOlympiaAccountAddressSample()

        override fun other(): LegacyOlympiaAccountAddress = newLegacyOlympiaAccountAddressSampleOther()
    }