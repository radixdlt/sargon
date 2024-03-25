package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.StakeClaim
import com.radixdlt.sargon.newStakeClaimSample
import com.radixdlt.sargon.newStakeClaimSampleOther

@VisibleForTesting
val StakeClaim.Companion.sample: Sample<StakeClaim>
    get() = object : Sample<StakeClaim> {
        override fun invoke(): StakeClaim = newStakeClaimSample()

        override fun other(): StakeClaim = newStakeClaimSampleOther()
    }

class StakeClaimPreviewParameterProvider :
    PreviewParameterProvider<StakeClaim> {
    override val values: Sequence<StakeClaim>
        get() = StakeClaim.sample.all.asSequence()

}