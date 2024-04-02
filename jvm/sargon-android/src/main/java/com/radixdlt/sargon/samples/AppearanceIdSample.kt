package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.AppearanceId
import com.radixdlt.sargon.newAppearanceIdSample
import com.radixdlt.sargon.newAppearanceIdSampleOther

@UsesSampleValues
val AppearanceId.Companion.sample: Sample<AppearanceId>
    get() = object : Sample<AppearanceId> {

        override fun invoke(): AppearanceId = newAppearanceIdSample()

        override fun other(): AppearanceId = newAppearanceIdSampleOther()
    }