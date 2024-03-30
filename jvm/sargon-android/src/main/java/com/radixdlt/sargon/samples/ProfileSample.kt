package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.newProfileSample
import com.radixdlt.sargon.newProfileSampleOther

@VisibleForTesting
val Profile.Companion.sample: Sample<Profile>
    get() = object : Sample<Profile> {

        override fun invoke(): Profile = newProfileSample()

        override fun other(): Profile = newProfileSampleOther()

    }