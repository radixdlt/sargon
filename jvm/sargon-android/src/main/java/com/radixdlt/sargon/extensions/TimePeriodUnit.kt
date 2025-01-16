package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.TimePeriodUnit
import com.radixdlt.sargon.constantMaxRecoveryConfirmationFallbackPeriodUnits

val TimePeriodUnit.values
    get() = (1..constantMaxRecoveryConfirmationFallbackPeriodUnits().toInt()).toList()