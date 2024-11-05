package com.radixdlt.sargon.samples

import com.radixdlt.sargon.CompiledSubintent
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newCompiledSubintentSample
import com.radixdlt.sargon.newCompiledSubintentSampleOther

@UsesSampleValues
val CompiledSubintent.Companion.sample
    get() = object : Sample<CompiledSubintent> {
        override fun invoke(): CompiledSubintent = newCompiledSubintentSample()

        override fun other(): CompiledSubintent = newCompiledSubintentSampleOther()
    }