package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AccountLockerClaimableResource
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newAccountLockerClaimableResourceSample
import com.radixdlt.sargon.newAccountLockerClaimableResourceSampleOther
import com.radixdlt.sargon.samples.Sample

@UsesSampleValues
val AccountLockerClaimableResource.Companion.sample: Sample<AccountLockerClaimableResource>
    get() = object : Sample<AccountLockerClaimableResource> {
        override fun invoke(): AccountLockerClaimableResource = newAccountLockerClaimableResourceSample()

        override fun other(): AccountLockerClaimableResource = newAccountLockerClaimableResourceSampleOther()
    }