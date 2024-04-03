package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.CompiledNotarizedIntent
import com.radixdlt.sargon.newCompiledNotarizedIntentSample
import com.radixdlt.sargon.newCompiledNotarizedIntentSampleOther

@UsesSampleValues
val CompiledNotarizedIntent.Companion.sample: Sample<CompiledNotarizedIntent>
    get() = object : Sample<CompiledNotarizedIntent> {

        override fun invoke(): CompiledNotarizedIntent = newCompiledNotarizedIntentSample()

        override fun other(): CompiledNotarizedIntent = newCompiledNotarizedIntentSampleOther()
    }