package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.PackageAddress
import com.radixdlt.sargon.newPackageAddressSampleMainnet
import com.radixdlt.sargon.newPackageAddressSampleMainnetOther

@VisibleForTesting
val PackageAddress.Companion.sample: Sample<PackageAddress>
    get() = object : Sample<PackageAddress> {
        
        override fun invoke(): PackageAddress = newPackageAddressSampleMainnet()

        override fun other(): PackageAddress = newPackageAddressSampleMainnetOther()

    }

class PackageAddressPreviewParameterProvider : PreviewParameterProvider<PackageAddress> {
    override val values: Sequence<PackageAddress>
        get() = PackageAddress.sample.all.asSequence()

}