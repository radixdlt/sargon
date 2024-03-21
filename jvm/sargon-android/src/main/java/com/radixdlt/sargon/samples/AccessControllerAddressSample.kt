package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.AccessControllerAddress
import com.radixdlt.sargon.newAccessControllerAddressSampleMainnet
import com.radixdlt.sargon.newAccessControllerAddressSampleMainnetOther
import com.radixdlt.sargon.newAccessControllerAddressSampleStokenet
import com.radixdlt.sargon.newAccessControllerAddressSampleStokenetOther

@VisibleForTesting
val AccessControllerAddress.Companion.sampleMainnet: Sample<AccessControllerAddress>
    get() = object : Sample<AccessControllerAddress> {

        override fun invoke(): AccessControllerAddress =
            newAccessControllerAddressSampleMainnet()

        override fun other(): AccessControllerAddress =
            newAccessControllerAddressSampleMainnetOther()

    }

@VisibleForTesting
val AccessControllerAddress.Companion.sampleStokenet: Sample<AccessControllerAddress>
    get() = object : Sample<AccessControllerAddress> {

        override fun invoke(): AccessControllerAddress =
            newAccessControllerAddressSampleStokenet()

        override fun other(): AccessControllerAddress =
            newAccessControllerAddressSampleStokenetOther()

    }

class AccessControllerAddressMainnetPreviewParameterProvider :
    PreviewParameterProvider<AccessControllerAddress> {
    override val values: Sequence<AccessControllerAddress>
        get() = AccessControllerAddress.sampleMainnet.all.asSequence()

}

class AccessControllerAddressStokenetPreviewParameterProvider :
    PreviewParameterProvider<AccessControllerAddress> {
    override val values: Sequence<AccessControllerAddress>
        get() = AccessControllerAddress.sampleStokenet.all.asSequence()

}