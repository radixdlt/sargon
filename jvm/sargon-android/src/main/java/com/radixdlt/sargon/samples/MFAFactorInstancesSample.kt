package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.MfaFactorInstances
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.newMFAFactorInstancesSample
import com.radixdlt.sargon.newMFAFactorInstancesSampleOther

@UsesSampleValues
val MfaFactorInstances.Companion.sample: Sample<MfaFactorInstances>
    get() = object : Sample<MfaFactorInstances> {

        override fun invoke(): MfaFactorInstances = newMFAFactorInstancesSample().asIdentifiable()

        override fun other(): MfaFactorInstances = newMFAFactorInstancesSampleOther().asIdentifiable()
    }