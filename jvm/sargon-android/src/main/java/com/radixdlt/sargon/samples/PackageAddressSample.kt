package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.PackageAddress
import com.radixdlt.sargon.newPackageAddressSampleMainnet
import com.radixdlt.sargon.newPackageAddressSampleMainnetOther
import com.radixdlt.sargon.newPackageAddressSampleStokenet
import com.radixdlt.sargon.newPackageAddressSampleStokenetOther

@VisibleForTesting
val PackageAddress.Companion.sampleMainnet: Sample<PackageAddress>
    get() = object : Sample<PackageAddress> {
        
        override fun invoke(): PackageAddress = newPackageAddressSampleMainnet()

        override fun other(): PackageAddress = newPackageAddressSampleMainnetOther()

    }

@VisibleForTesting
val PackageAddress.Companion.sampleStokenet: Sample<PackageAddress>
    get() = object : Sample<PackageAddress> {

        override fun invoke(): PackageAddress = newPackageAddressSampleStokenet()

        override fun other(): PackageAddress = newPackageAddressSampleStokenetOther()

    }

class PackageAddressStokenetwPreviewParameterProvider : PreviewParameterProvider<PackageAddress> {
    override val values: Sequence<PackageAddress>
        get() = PackageAddress.sampleMainnet.all.asSequence()

}