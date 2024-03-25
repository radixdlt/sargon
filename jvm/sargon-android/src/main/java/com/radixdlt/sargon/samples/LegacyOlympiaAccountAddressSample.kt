package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.LegacyOlympiaAccountAddress
import com.radixdlt.sargon.newLegacyOlympiaAccountAddressSample
import com.radixdlt.sargon.newLegacyOlympiaAccountAddressSampleOther

@VisibleForTesting
val LegacyOlympiaAccountAddress.Companion.sample: Sample<LegacyOlympiaAccountAddress>
    get() = object : Sample<LegacyOlympiaAccountAddress> {
        override fun invoke(): LegacyOlympiaAccountAddress = newLegacyOlympiaAccountAddressSample()

        override fun other(): LegacyOlympiaAccountAddress = newLegacyOlympiaAccountAddressSampleOther()
    }

class LegacyOlympiaAccountAddressPreviewParameterProvider :
    PreviewParameterProvider<LegacyOlympiaAccountAddress> {
    override val values: Sequence<LegacyOlympiaAccountAddress>
        get() = LegacyOlympiaAccountAddress.sample.all.asSequence()
}