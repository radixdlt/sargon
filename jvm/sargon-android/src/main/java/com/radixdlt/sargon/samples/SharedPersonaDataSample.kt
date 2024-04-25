package com.radixdlt.sargon.samples

import com.radixdlt.sargon.SharedPersonaData
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newSharedPersonaDataSample
import com.radixdlt.sargon.newSharedPersonaDataSampleOther

@UsesSampleValues
val SharedPersonaData.Companion.sample: Sample<SharedPersonaData>
    get() = object : Sample<SharedPersonaData> {
        override fun invoke(): SharedPersonaData = newSharedPersonaDataSample()

        override fun other(): SharedPersonaData = newSharedPersonaDataSampleOther()

    }