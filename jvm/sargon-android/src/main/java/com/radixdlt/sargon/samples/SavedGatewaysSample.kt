package com.radixdlt.sargon.samples

import com.radixdlt.sargon.SavedGateways
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newSavedGatewaysSample
import com.radixdlt.sargon.newSavedGatewaysSampleOther

@UsesSampleValues
val SavedGateways.Companion.sample: Sample<SavedGateways>
    get() = object : Sample<SavedGateways> {

        override fun invoke(): SavedGateways = newSavedGatewaysSample()

        override fun other(): SavedGateways = newSavedGatewaysSampleOther()
    }