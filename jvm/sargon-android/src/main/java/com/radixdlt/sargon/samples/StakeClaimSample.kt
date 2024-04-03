package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.StakeClaim
import com.radixdlt.sargon.newStakeClaimSample
import com.radixdlt.sargon.newStakeClaimSampleOther

@UsesSampleValues
val StakeClaim.Companion.sample: Sample<StakeClaim>
    get() = object : Sample<StakeClaim> {
        override fun invoke(): StakeClaim = newStakeClaimSample()

        override fun other(): StakeClaim = newStakeClaimSampleOther()
    }