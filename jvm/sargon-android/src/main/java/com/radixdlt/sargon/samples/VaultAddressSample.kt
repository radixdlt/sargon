package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.VaultAddress
import com.radixdlt.sargon.newVaultAddressSampleMainnetFungible
import com.radixdlt.sargon.newVaultAddressSampleMainnetNonFungible
import com.radixdlt.sargon.newVaultAddressSampleStokenetFungible
import com.radixdlt.sargon.newVaultAddressSampleStokenetNonFungible

@VisibleForTesting(otherwise = VisibleForTesting.PACKAGE_PRIVATE)
val VaultAddress.Companion.sampleMainnet: Sample<VaultAddress>
    get() = object : Sample<VaultAddress> {

        override fun invoke(): VaultAddress = newVaultAddressSampleMainnetFungible()

        override fun other(): VaultAddress = newVaultAddressSampleMainnetNonFungible()
    }

@VisibleForTesting(otherwise = VisibleForTesting.PACKAGE_PRIVATE)
val VaultAddress.Companion.sampleStokenet: Sample<VaultAddress>
    get() = object : Sample<VaultAddress> {

        override fun invoke(): VaultAddress = newVaultAddressSampleStokenetFungible()

        override fun other(): VaultAddress = newVaultAddressSampleStokenetNonFungible()
    }

class VaultAddressMainnetPreviewParameterProvider: PreviewParameterProvider<VaultAddress> {
    override val values: Sequence<VaultAddress>
        get() = VaultAddress.sampleMainnet.all.asSequence()

}

class VaultAddressStokenetPreviewParameterProvider: PreviewParameterProvider<VaultAddress> {
    override val values: Sequence<VaultAddress>
        get() = VaultAddress.sampleStokenet.all.asSequence()

}