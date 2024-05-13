package com.radixdlt.sargon.samples

import com.radixdlt.sargon.LinkConnectionQrData
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newLinkConnectionQrDataSample
import com.radixdlt.sargon.newLinkConnectionQrDataSampleOther

@UsesSampleValues
val LinkConnectionQrData.Companion.sample: Sample<LinkConnectionQrData>
    get() = object : Sample<LinkConnectionQrData> {

        override fun invoke(): LinkConnectionQrData = newLinkConnectionQrDataSample()

        override fun other(): LinkConnectionQrData = newLinkConnectionQrDataSampleOther()
    }