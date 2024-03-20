package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.PoolAddress
import com.radixdlt.sargon.newPoolAddressSampleMainnetMulti
import com.radixdlt.sargon.newPoolAddressSampleMainnetSingle
import com.radixdlt.sargon.newPoolAddressSampleMainnetTwo
import com.radixdlt.sargon.newPoolAddressSampleStokenetMulti
import com.radixdlt.sargon.newPoolAddressSampleStokenetSingle
import com.radixdlt.sargon.newPoolAddressSampleStokenetTwo

@VisibleForTesting
object PoolAddressSampleMainnet: Sample<PoolAddress> {
    override fun invoke(): PoolAddress = single

    override fun other(): PoolAddress = two

    val single: PoolAddress
        get() = newPoolAddressSampleMainnetSingle()

    val two: PoolAddress
        get() = newPoolAddressSampleMainnetTwo()

    val multi: PoolAddress
        get() = newPoolAddressSampleMainnetMulti()
}

@VisibleForTesting
object PoolAddressSampleStokenet: Sample<PoolAddress> {
    override fun invoke(): PoolAddress = single

    override fun other(): PoolAddress = two

    val single: PoolAddress
        get() = newPoolAddressSampleStokenetSingle()

    val two: PoolAddress
        get() = newPoolAddressSampleStokenetTwo()

    val multi: PoolAddress
        get() = newPoolAddressSampleStokenetMulti()
}

@VisibleForTesting
val PoolAddress.Companion.sampleMainnet: PoolAddressSampleMainnet
    get() = PoolAddressSampleMainnet

@VisibleForTesting
val PoolAddress.Companion.sampleStokenet: PoolAddressSampleStokenet
    get() = PoolAddressSampleStokenet

class PoolAddressStokenetPreviewParameterProvider: PreviewParameterProvider<PoolAddress> {
    override val values: Sequence<PoolAddress>
        get() = PoolAddress.sampleStokenet.all.asSequence()

}