package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.AddressOfAccountOrPersona
import com.radixdlt.sargon.ManifestEncounteredAddress
import com.radixdlt.sargon.newAddressOfAccountOrPersonaSampleMainnet
import com.radixdlt.sargon.newAddressOfAccountOrPersonaSampleMainnetOther
import com.radixdlt.sargon.newAddressOfAccountOrPersonaSampleStokenet
import com.radixdlt.sargon.newAddressOfAccountOrPersonaSampleStokenetOther
import com.radixdlt.sargon.newManifestEncounteredAddressSampleMainnet
import com.radixdlt.sargon.newManifestEncounteredAddressSampleMainnetOther
import com.radixdlt.sargon.newManifestEncounteredAddressSampleStokenet
import com.radixdlt.sargon.newManifestEncounteredAddressSampleStokenetOther

@UsesSampleValues
val ManifestEncounteredAddress.Companion.sampleMainnet: Sample<ManifestEncounteredAddress>
    get() = object : Sample<ManifestEncounteredAddress> {
        override fun invoke(): ManifestEncounteredAddress = newManifestEncounteredAddressSampleMainnet()

        override fun other(): ManifestEncounteredAddress = newManifestEncounteredAddressSampleMainnetOther()
    }

@UsesSampleValues
val ManifestEncounteredAddress.Companion.sampleStokenet: Sample<ManifestEncounteredAddress>
    get() = object : Sample<ManifestEncounteredAddress> {
        override fun invoke(): ManifestEncounteredAddress = newManifestEncounteredAddressSampleStokenet()

        override fun other(): ManifestEncounteredAddress = newManifestEncounteredAddressSampleStokenetOther()
    }