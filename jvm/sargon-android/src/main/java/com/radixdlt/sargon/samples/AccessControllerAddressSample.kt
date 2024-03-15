package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.AccessControllerAddress
import com.radixdlt.sargon.newAccessControllerAddressSampleMainnet
import com.radixdlt.sargon.newAccessControllerAddressSampleMainnetOther

@VisibleForTesting
val AccessControllerAddress.Companion.sample: Sample<AccessControllerAddress>
    get() = object : Sample<AccessControllerAddress> {

        override fun invoke(): AccessControllerAddress =
            newAccessControllerAddressSampleMainnet()

        override fun other(): AccessControllerAddress =
            newAccessControllerAddressSampleMainnetOther()

    }

class AccessControllerAddressPreviewParameterProvider :
    PreviewParameterProvider<AccessControllerAddress> {
    override val values: Sequence<AccessControllerAddress>
        get() = AccessControllerAddress.sample.all.asSequence()

}