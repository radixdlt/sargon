package com.radixdlt.sargon.samples

import com.radixdlt.sargon.HdPathComponent
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newHdPathComponentSample
import com.radixdlt.sargon.newHdPathComponentSampleOther

@UsesSampleValues
val HdPathComponent.Companion.sample: Sample<HdPathComponent>
    get() = object : Sample<HdPathComponent> {
        override fun invoke(): HdPathComponent = newHdPathComponentSample()

        override fun other(): HdPathComponent = newHdPathComponentSampleOther()
    }