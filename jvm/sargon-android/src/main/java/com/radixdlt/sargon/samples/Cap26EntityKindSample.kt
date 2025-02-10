package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Cap26EntityKind
import com.radixdlt.sargon.annotation.UsesSampleValues

@UsesSampleValues
val Cap26EntityKind.Companion.sample: Sample<Cap26EntityKind>
    get() = object : Sample<Cap26EntityKind> {
        override fun invoke(): Cap26EntityKind = Cap26EntityKind.ACCOUNT

        override fun other(): Cap26EntityKind = Cap26EntityKind.IDENTITY

    }