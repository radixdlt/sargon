package com.radixdlt.sargon.samples

import com.radixdlt.sargon.EntityFlag
import com.radixdlt.sargon.EntityFlags
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.init

@UsesSampleValues
val EntityFlags.Companion.sample: Sample<EntityFlags>
    get() = object: Sample<EntityFlags> {
        override fun invoke(): EntityFlags = EntityFlags.init(EntityFlag.DELETED_BY_USER)

        override fun other(): EntityFlags = EntityFlags.init(
            EntityFlag.DELETED_BY_USER,
            EntityFlag.PLACEHOLDER_SAMPLE_VALUE_FLAG
        )

    }