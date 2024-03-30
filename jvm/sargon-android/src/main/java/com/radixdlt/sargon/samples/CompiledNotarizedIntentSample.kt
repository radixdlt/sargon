package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.CompiledNotarizedIntent
import com.radixdlt.sargon.newCompiledNotarizedIntentSample
import com.radixdlt.sargon.newCompiledNotarizedIntentSampleOther

@VisibleForTesting
val CompiledNotarizedIntent.Companion.sample: Sample<CompiledNotarizedIntent>
    get() = object : Sample<CompiledNotarizedIntent> {

        override fun invoke(): CompiledNotarizedIntent = newCompiledNotarizedIntentSample()

        override fun other(): CompiledNotarizedIntent = newCompiledNotarizedIntentSampleOther()
    }