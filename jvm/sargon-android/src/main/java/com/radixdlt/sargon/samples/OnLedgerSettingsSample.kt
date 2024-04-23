package com.radixdlt.sargon.samples

import com.radixdlt.sargon.OnLedgerSettings
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newOnLedgerSettingsSample
import com.radixdlt.sargon.newOnLedgerSettingsSampleOther

@UsesSampleValues
val OnLedgerSettings.Companion.sample: Sample<OnLedgerSettings>
    get() = object : Sample<OnLedgerSettings> {
        override fun invoke(): OnLedgerSettings = newOnLedgerSettingsSample()

        override fun other(): OnLedgerSettings = newOnLedgerSettingsSampleOther()

    }