package com.radixdlt.sargon.samples

import com.radixdlt.sargon.RequestedQuantity
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newRequestedQuantitySample
import com.radixdlt.sargon.newRequestedQuantitySampleOther

@UsesSampleValues
val RequestedQuantity.Companion.sample: Sample<RequestedQuantity>
    get() = object : Sample<RequestedQuantity> {
        override fun invoke(): RequestedQuantity = newRequestedQuantitySample()

        override fun other(): RequestedQuantity = newRequestedQuantitySampleOther()

    }