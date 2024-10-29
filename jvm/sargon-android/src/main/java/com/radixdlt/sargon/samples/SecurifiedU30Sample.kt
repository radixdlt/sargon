package com.radixdlt.sargon.samples

import com.radixdlt.sargon.SecurifiedU30
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newSecurifiedSample
import com.radixdlt.sargon.newSecurifiedSampleOther

@UsesSampleValues
val SecurifiedU30.Companion.sample: Sample<SecurifiedU30>
    get() = object : Sample<SecurifiedU30> {
        override fun invoke(): SecurifiedU30 = newSecurifiedSample()

        override fun other(): SecurifiedU30 = newSecurifiedSampleOther()
    }