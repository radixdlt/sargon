package com.radixdlt.sargon.samples

import com.radixdlt.sargon.DepositRule
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newDepositRuleSample
import com.radixdlt.sargon.newDepositRuleSampleOther

@UsesSampleValues
val DepositRule.Companion.sample: Sample<DepositRule>
    get() = object : Sample<DepositRule> {
        override fun invoke(): DepositRule = newDepositRuleSample()

        override fun other(): DepositRule = newDepositRuleSampleOther()

    }