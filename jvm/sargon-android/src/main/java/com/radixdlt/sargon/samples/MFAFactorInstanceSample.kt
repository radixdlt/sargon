package com.radixdlt.sargon.samples

import com.radixdlt.sargon.MfaFactorInstance
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newMfaFactorInstanceSample
import com.radixdlt.sargon.newMfaFactorInstanceSampleOther

@UsesSampleValues
val MfaFactorInstance.Companion.sample: Sample<MfaFactorInstance>
    get() = object : Sample<MfaFactorInstance> {

        override fun invoke(): MfaFactorInstance = newMfaFactorInstanceSample()

        override fun other(): MfaFactorInstance = newMfaFactorInstanceSampleOther()
    }