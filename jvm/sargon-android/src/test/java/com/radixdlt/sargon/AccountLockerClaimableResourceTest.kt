package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.sample
import com.radixdlt.sargon.samples.Sample

class AccountLockerClaimableResourceTest : SampleTestable<AccountLockerClaimableResource> {

    override val samples: List<Sample<AccountLockerClaimableResource>>
        get() = listOf(AccountLockerClaimableResource.sample)
}