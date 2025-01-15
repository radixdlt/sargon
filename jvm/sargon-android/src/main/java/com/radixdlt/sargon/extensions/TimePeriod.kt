package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.TimePeriod
import com.radixdlt.sargon.newTimePeriodWithDays
import com.radixdlt.sargon.timePeriodToDays

fun TimePeriod.Companion.withDays(value: Int): TimePeriod = newTimePeriodWithDays(value.toUShort())

val TimePeriod.days
    get() = timePeriodToDays(this)