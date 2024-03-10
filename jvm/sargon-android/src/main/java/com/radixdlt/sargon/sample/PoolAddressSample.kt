package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.PoolAddress
import com.radixdlt.sargon.newPoolAddressSampleMulti
import com.radixdlt.sargon.newPoolAddressSampleSingle
import com.radixdlt.sargon.newPoolAddressSampleTwo

@VisibleForTesting
val PoolAddress.Companion.sample: Sample<PoolAddress>
    get() = object : Sample<PoolAddress> {

        override fun invoke(): PoolAddress = mainnetSingle

        override fun other(): PoolAddress = mainnetTwo

        val mainnetSingle: PoolAddress
            get() = newPoolAddressSampleSingle()

        val mainnetTwo: PoolAddress
            get() = newPoolAddressSampleTwo()

        val mainnetMulti: PoolAddress
            get() = newPoolAddressSampleMulti()

    }

class PoolAddressPreviewParameterProvider: PreviewParameterProvider<PoolAddress> {
    override val values: Sequence<PoolAddress>
        get() = PoolAddress.sample.all.asSequence()

}