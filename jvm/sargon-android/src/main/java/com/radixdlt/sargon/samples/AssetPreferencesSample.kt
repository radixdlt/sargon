package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.AssetPreferences
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.newAssetPreferencesSample
import com.radixdlt.sargon.newAssetPreferencesSampleOther

@UsesSampleValues
val AssetPreferences.Companion.sample: Sample<AssetPreferences>
    get() = object : Sample<AssetPreferences> {

        override fun invoke(): AssetPreferences = newAssetPreferencesSample().asIdentifiable()

        override fun other(): AssetPreferences = newAssetPreferencesSampleOther().asIdentifiable()
    }