package com.radixdlt.sargon

import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample

class AppPreferencesTest: SampleTestable<AppPreferences> {

    override val samples: List<Sample<AppPreferences>>
        get() = listOf(AppPreferences.sample)

}