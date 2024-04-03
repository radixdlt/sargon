package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.Decimal192
import com.radixdlt.sargon.extensions.MAX
import com.radixdlt.sargon.extensions.toDecimal192

@UsesSampleValues
val Decimal192.Companion.sample: Sample<Decimal192>
    get() = object : Sample<Decimal192> {

        override fun invoke(): Decimal192 = 123456789.toDecimal192()

        override fun other(): Decimal192 = Decimal192.MAX
    }