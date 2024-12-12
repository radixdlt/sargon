package com.radixdlt.sargon.samples

import com.radixdlt.sargon.RolaChallenge
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newRolaChallengeSample
import com.radixdlt.sargon.newRolaChallengeSampleOther

@UsesSampleValues
val RolaChallenge.Companion.sample: Sample<RolaChallenge>
    get() = object : Sample<RolaChallenge> {
        override fun invoke(): RolaChallenge = newRolaChallengeSample()

        override fun other(): RolaChallenge = newRolaChallengeSampleOther()
    }