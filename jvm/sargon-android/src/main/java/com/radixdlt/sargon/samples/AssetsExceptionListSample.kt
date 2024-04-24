package com.radixdlt.sargon.samples

import com.radixdlt.sargon.AssetsExceptionList
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newAssetsExceptionListSample
import com.radixdlt.sargon.newAssetsExceptionListSampleOther

@UsesSampleValues
val AssetsExceptionList.Companion.sample: Sample<AssetsExceptionList>
    get() = object : Sample<AssetsExceptionList> {
        override fun invoke(): AssetsExceptionList = newAssetsExceptionListSample()

        override fun other(): AssetsExceptionList = newAssetsExceptionListSampleOther()

    }