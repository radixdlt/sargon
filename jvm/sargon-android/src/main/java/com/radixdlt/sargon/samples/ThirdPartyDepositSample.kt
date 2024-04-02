package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.ThirdPartyDeposits
import com.radixdlt.sargon.newThirdPartyDepositsSample
import com.radixdlt.sargon.newThirdPartyDepositsSampleOther

@UsesSampleValues
val ThirdPartyDeposits.Companion.sample: Sample<ThirdPartyDeposits>
    get() = object : Sample<ThirdPartyDeposits> {
        override fun invoke(): ThirdPartyDeposits = newThirdPartyDepositsSample()

        override fun other(): ThirdPartyDeposits = newThirdPartyDepositsSampleOther()

    }