package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.AppearanceId
import com.radixdlt.sargon.newAppearanceIdSample
import com.radixdlt.sargon.newAppearanceIdSampleOther

@VisibleForTesting
val AppearanceId.Companion.sample: Sample<AppearanceId>
    get() = object : Sample<AppearanceId> {

        override fun invoke(): AppearanceId = newAppearanceIdSample()

        override fun other(): AppearanceId = newAppearanceIdSampleOther()
    }