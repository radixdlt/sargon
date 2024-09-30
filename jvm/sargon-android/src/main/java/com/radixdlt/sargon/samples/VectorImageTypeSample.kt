package com.radixdlt.sargon.samples

import com.radixdlt.sargon.VectorImageType
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newVectorImageTypeSample
import com.radixdlt.sargon.newVectorImageTypeSampleOther

@UsesSampleValues
val VectorImageType.Companion.sample: Sample<VectorImageType>
    get() = object : Sample<VectorImageType> {
        override fun invoke(): VectorImageType = newVectorImageTypeSample()

        override fun other(): VectorImageType = newVectorImageTypeSampleOther()
    }