package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.ValidatorAddress
import com.radixdlt.sargon.VaultAddress
import com.radixdlt.sargon.newValidatorAddressRandom
import com.radixdlt.sargon.newVaultAddressSampleMainnetFungible
import com.radixdlt.sargon.newVaultAddressSampleMainnetNonFungible
import com.radixdlt.sargon.newVaultAddressRandom
import com.radixdlt.sargon.newVaultAddressSampleStokenetFungible
import com.radixdlt.sargon.newVaultAddressSampleStokenetNonFungible

@UsesSampleValues
object VaultAddressSampleMainnet: SampleWithRandomValues<VaultAddress> {
    override fun invoke(): VaultAddress = newVaultAddressSampleMainnetFungible()

    override fun other(): VaultAddress = newVaultAddressSampleMainnetNonFungible()

    override fun random(): VaultAddress = newVaultAddressRandom(networkId = NetworkId.MAINNET)
}

@UsesSampleValues
val VaultAddress.Companion.sampleMainnet: VaultAddressSampleMainnet
    get() = VaultAddressSampleMainnet

@UsesSampleValues
object VaultAddressSampleStokenet: SampleWithRandomValues<VaultAddress> {
    override fun invoke(): VaultAddress = newVaultAddressSampleStokenetFungible()

    override fun other(): VaultAddress = newVaultAddressSampleStokenetNonFungible()

    override fun random(): VaultAddress = newVaultAddressRandom(
        networkId = NetworkId.STOKENET
    )
}

@UsesSampleValues
val VaultAddress.Companion.sampleStokenet: VaultAddressSampleStokenet
    get() = VaultAddressSampleStokenet

@UsesSampleValues
fun VaultAddress.Companion.sampleRandom(
    networkId: NetworkId
) = newVaultAddressRandom(networkId = networkId)