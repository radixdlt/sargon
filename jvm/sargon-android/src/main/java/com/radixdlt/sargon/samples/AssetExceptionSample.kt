package com.radixdlt.sargon.samples

import com.radixdlt.sargon.AssetException
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newAssetExceptionSample
import com.radixdlt.sargon.newAssetExceptionSampleOther

@UsesSampleValues
val AssetException.Companion.sample: Sample<AssetException>
    get() = object : Sample<AssetException> {
        override fun invoke(): AssetException = newAssetExceptionSample()

        override fun other(): AssetException = newAssetExceptionSampleOther()
    }