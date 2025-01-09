package com.radixdlt.sargon.samples

import com.radixdlt.sargon.AccountForDisplay
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newAccountForDisplaySample
import com.radixdlt.sargon.newAccountForDisplaySampleOther

@UsesSampleValues
val AccountForDisplay.Companion.sample: Sample<AccountForDisplay>
    get() = object : Sample<AccountForDisplay> {
        override fun invoke(): AccountForDisplay = newAccountForDisplaySample()

        override fun other(): AccountForDisplay = newAccountForDisplaySampleOther()
    }