package com.radixdlt.sargon.samples

import com.radixdlt.sargon.PasswordFactorSource
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newPasswordFactorSourceSample
import com.radixdlt.sargon.newPasswordFactorSourceSampleOther

@UsesSampleValues
val PasswordFactorSource.Companion.sample: Sample<PasswordFactorSource>
    get() = object : Sample<PasswordFactorSource> {
        override fun invoke(): PasswordFactorSource = newPasswordFactorSourceSample()

        override fun other(): PasswordFactorSource = newPasswordFactorSourceSampleOther()
    }