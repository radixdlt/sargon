
package com.radixdlt.sargon.samples

import com.radixdlt.sargon.ResourceSpecifier
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newResourceSpecifierSample
import com.radixdlt.sargon.newResourceSpecifierSampleOther

@UsesSampleValues
val ResourceSpecifier.Companion.sample: Sample<ResourceSpecifier>
    get() = object: Sample<ResourceSpecifier> {
        override fun invoke(): ResourceSpecifier = newResourceSpecifierSample()

        override fun other(): ResourceSpecifier = newResourceSpecifierSampleOther()
    }
