package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.TimePeriodUnit
import com.radixdlt.sargon.timePeriodUnitValues

val TimePeriodUnit.values
    get() = timePeriodUnitValues(this).toList()
        .map { it.toInt() }