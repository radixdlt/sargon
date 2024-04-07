package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newFactorSourcesSample
import com.radixdlt.sargon.newFactorSourcesSampleOther

@UsesSampleValues
fun factorSourcesSample() = newFactorSourcesSample()

@UsesSampleValues
fun factorSourcesSampleOther() = newFactorSourcesSampleOther()