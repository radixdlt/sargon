package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.TimePeriod
import com.radixdlt.sargon.newTimePeriodWithDays
import com.radixdlt.sargon.timePeriodToDays

fun TimePeriod.Companion.init(days: Int): TimePeriod = newTimePeriodWithDays(days.toUShort())

val TimePeriod.days
    get() = timePeriodToDays(this).toInt()