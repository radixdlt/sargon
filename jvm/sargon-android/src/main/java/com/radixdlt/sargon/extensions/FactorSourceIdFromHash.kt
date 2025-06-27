package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.SpotCheckInput
import com.radixdlt.sargon.factorSourceIdFromHashPerformSpotCheck

fun FactorSourceIdFromHash.spotCheck(
    input: SpotCheckInput
) = factorSourceIdFromHashPerformSpotCheck(this, input)