package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.DisplayName
import com.radixdlt.sargon.newDisplayNameSample
import com.radixdlt.sargon.newDisplayNameSampleOther

@UsesSampleValues
val DisplayName.Companion.sample: Sample<DisplayName>
    get() = object : Sample<DisplayName> {

        override fun invoke(): DisplayName = newDisplayNameSample()

        override fun other(): DisplayName = newDisplayNameSampleOther()
    }