package com.radixdlt.sargon.samples

import com.radixdlt.sargon.EntityFlag
import com.radixdlt.sargon.annotation.UsesSampleValues

@UsesSampleValues
val EntityFlag.Companion.sample: Sample<EntityFlag>
    get() = object: Sample<EntityFlag> {
        override fun invoke(): EntityFlag = EntityFlag.DELETED_BY_USER

        override fun other(): EntityFlag = EntityFlag.PLACEHOLDER_SAMPLE_VALUE_FLAG

    }