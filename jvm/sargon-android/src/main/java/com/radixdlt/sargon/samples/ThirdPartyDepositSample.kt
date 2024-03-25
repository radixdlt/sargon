package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.ThirdPartyDeposits
import com.radixdlt.sargon.newThirdPartyDepositsSample
import com.radixdlt.sargon.newThirdPartyDepositsSampleOther

@VisibleForTesting
val ThirdPartyDeposits.Companion.sample: Sample<ThirdPartyDeposits>
    get() = object : Sample<ThirdPartyDeposits> {
        override fun invoke(): ThirdPartyDeposits = newThirdPartyDepositsSample()

        override fun other(): ThirdPartyDeposits = newThirdPartyDepositsSampleOther()

    }

class ThirdPartyDepositsPreviewParameterProvider: PreviewParameterProvider<ThirdPartyDeposits> {
    override val values: Sequence<ThirdPartyDeposits>
        get() = ThirdPartyDeposits.sample.all.asSequence()

}