package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.PoolAddress
import com.radixdlt.sargon.newPoolAddressSampleMainnetMulti
import com.radixdlt.sargon.newPoolAddressSampleMainnetSingle
import com.radixdlt.sargon.newPoolAddressSampleMainnetTwo

@VisibleForTesting
object PoolAddressSample: Sample<PoolAddress> {
    override fun invoke(): PoolAddress = mainnetSingle

    override fun other(): PoolAddress = mainnetTwo

    val mainnetSingle: PoolAddress
        get() = newPoolAddressSampleMainnetSingle()

    val mainnetTwo: PoolAddress
        get() = newPoolAddressSampleMainnetTwo()

    val mainnetMulti: PoolAddress
        get() = newPoolAddressSampleMainnetMulti()
}

@VisibleForTesting
val PoolAddress.Companion.sample: PoolAddressSample
    get() = PoolAddressSample

class PoolAddressPreviewParameterProvider: PreviewParameterProvider<PoolAddress> {
    override val values: Sequence<PoolAddress>
        get() = PoolAddress.sample.all.asSequence()

}