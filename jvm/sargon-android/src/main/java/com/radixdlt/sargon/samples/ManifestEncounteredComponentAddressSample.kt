package com.radixdlt.sargon.samples

import com.radixdlt.sargon.ManifestEncounteredComponentAddress
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newManifestEncounteredComponentAddressSampleMainnet
import com.radixdlt.sargon.newManifestEncounteredComponentAddressSampleMainnetOther
import com.radixdlt.sargon.newManifestEncounteredComponentAddressSampleStokenet
import com.radixdlt.sargon.newManifestEncounteredComponentAddressSampleStokenetOther

@UsesSampleValues
val ManifestEncounteredComponentAddress.Companion.sampleMainnet: Sample<ManifestEncounteredComponentAddress>
    get() = object : Sample<ManifestEncounteredComponentAddress> {
        override fun invoke(): ManifestEncounteredComponentAddress = newManifestEncounteredComponentAddressSampleMainnet()

        override fun other(): ManifestEncounteredComponentAddress = newManifestEncounteredComponentAddressSampleMainnetOther()
    }

@UsesSampleValues
val ManifestEncounteredComponentAddress.Companion.sampleStokenet: Sample<ManifestEncounteredComponentAddress>
    get() = object : Sample<ManifestEncounteredComponentAddress> {
        override fun invoke(): ManifestEncounteredComponentAddress = newManifestEncounteredComponentAddressSampleStokenet()

        override fun other(): ManifestEncounteredComponentAddress = newManifestEncounteredComponentAddressSampleStokenetOther()
    }