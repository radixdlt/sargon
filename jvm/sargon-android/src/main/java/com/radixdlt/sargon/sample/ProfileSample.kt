package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.newProfileSample
import com.radixdlt.sargon.newProfileSampleOther

@VisibleForTesting
val Profile.Companion.sample: Sample<Profile>
    get() = object : Sample<Profile> {

        override fun invoke(): Profile = newProfileSample()

        override fun other(): Profile = newProfileSampleOther()

    }

class ProfilePreviewParameterProvider : PreviewParameterProvider<Profile> {
    override val values: Sequence<Profile>
        get() = Profile.sample.all.asSequence()

}