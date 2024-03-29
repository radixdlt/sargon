package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.VaultAddress
import com.radixdlt.sargon.newVaultAddressSampleMainnetFungible
import com.radixdlt.sargon.newVaultAddressSampleMainnetNonFungible
import com.radixdlt.sargon.newVaultAddressSampleRandom
import com.radixdlt.sargon.newVaultAddressSampleStokenetFungible
import com.radixdlt.sargon.newVaultAddressSampleStokenetNonFungible

@VisibleForTesting
object VaultAddressSampleMainnet: SampleWithRandomValues<VaultAddress> {
    override fun invoke(): VaultAddress = newVaultAddressSampleMainnetFungible()

    override fun other(): VaultAddress = newVaultAddressSampleMainnetNonFungible()

    override fun random(): VaultAddress = newVaultAddressSampleRandom(networkId = NetworkId.MAINNET)
}

@VisibleForTesting
val VaultAddress.Companion.sampleMainnet: VaultAddressSampleMainnet
    get() = VaultAddressSampleMainnet

@VisibleForTesting
object VaultAddressSampleStokenet: SampleWithRandomValues<VaultAddress> {
    override fun invoke(): VaultAddress = newVaultAddressSampleStokenetFungible()

    override fun other(): VaultAddress = newVaultAddressSampleStokenetNonFungible()

    override fun random(): VaultAddress = newVaultAddressSampleRandom(
        networkId = NetworkId.STOKENET
    )
}

@VisibleForTesting
val VaultAddress.Companion.sampleStokenet: VaultAddressSampleStokenet
    get() = VaultAddressSampleStokenet