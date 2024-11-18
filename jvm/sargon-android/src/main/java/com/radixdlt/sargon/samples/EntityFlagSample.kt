package com.radixdlt.sargon.samples

import com.radixdlt.sargon.EntityFlag
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newEntityFlagSample
import com.radixdlt.sargon.newEntityFlagSampleOther

@UsesSampleValues
val EntityFlag.Companion.sample: Sample<EntityFlag>
    get() = object: Sample<EntityFlag> {
        override fun invoke(): EntityFlag = newEntityFlagSample()

        override fun other(): EntityFlag = newEntityFlagSampleOther()

    }