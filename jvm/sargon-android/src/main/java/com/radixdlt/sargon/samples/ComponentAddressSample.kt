package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.ComponentAddress
import com.radixdlt.sargon.newComponentAddressSampleMainnetGlobal
import com.radixdlt.sargon.newComponentAddressSampleMainnetInternal
import com.radixdlt.sargon.newComponentAddressSampleStokenetGlobal
import com.radixdlt.sargon.newComponentAddressSampleStokenetInternal

@UsesSampleValues
val ComponentAddress.Companion.sampleMainnet: Sample<ComponentAddress>
    get() = object : Sample<ComponentAddress> {

        override fun invoke(): ComponentAddress = newComponentAddressSampleMainnetGlobal()

        override fun other(): ComponentAddress = newComponentAddressSampleMainnetInternal()
    }

@UsesSampleValues
val ComponentAddress.Companion.sampleStokenet: Sample<ComponentAddress>
    get() = object : Sample<ComponentAddress> {

        override fun invoke(): ComponentAddress = newComponentAddressSampleStokenetGlobal()

        override fun other(): ComponentAddress = newComponentAddressSampleStokenetInternal()
    }