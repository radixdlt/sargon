package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.AccessControllerAddress
import com.radixdlt.sargon.newAccessControllerAddressSample
import com.radixdlt.sargon.newAccessControllerAddressSampleOther

@VisibleForTesting
val AccessControllerAddress.Companion.sample: Sample<AccessControllerAddress>
    get() = object : Sample<AccessControllerAddress> {

        override fun invoke(): AccessControllerAddress =
            newAccessControllerAddressSample()

        override fun other(): AccessControllerAddress =
            newAccessControllerAddressSampleOther()

    }

class AccessControllerAddressPreviewParameterProvider :
    PreviewParameterProvider<AccessControllerAddress> {
    override val values: Sequence<AccessControllerAddress>
        get() = AccessControllerAddress.sample.all.asSequence()

}