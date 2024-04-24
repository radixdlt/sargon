package com.radixdlt.sargon.samples

import com.radixdlt.sargon.DepositorsAllowList
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newDepositorsAllowListSample
import com.radixdlt.sargon.newDepositorsAllowListSampleOther

@UsesSampleValues
val DepositorsAllowList.Companion.sample: Sample<DepositorsAllowList>
    get() = object : Sample<DepositorsAllowList> {
        override fun invoke(): DepositorsAllowList = newDepositorsAllowListSample()

        override fun other(): DepositorsAllowList = newDepositorsAllowListSampleOther()
    }