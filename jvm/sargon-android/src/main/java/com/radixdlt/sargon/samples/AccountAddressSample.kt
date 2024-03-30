package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.AccessControllerAddress
import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.newAccessControllerAddressRandom
import com.radixdlt.sargon.newAccountAddressSampleMainnet
import com.radixdlt.sargon.newAccountAddressSampleMainnetOther
import com.radixdlt.sargon.newAccountAddressRandom
import com.radixdlt.sargon.newAccountAddressSampleStokenet
import com.radixdlt.sargon.newAccountAddressSampleStokenetOther

@VisibleForTesting
object AccountAddressSampleMainnet: SampleWithRandomValues<AccountAddress> {
    override fun invoke(): AccountAddress = newAccountAddressSampleMainnet()

    override fun other(): AccountAddress = newAccountAddressSampleMainnetOther()

    override fun random(): AccountAddress = newAccountAddressRandom(networkId = NetworkId.MAINNET)
}

@VisibleForTesting
val AccountAddress.Companion.sampleMainnet: AccountAddressSampleMainnet
    get() = AccountAddressSampleMainnet

@VisibleForTesting
object AccountAddressSampleStokenet: SampleWithRandomValues<AccountAddress> {
    override fun invoke(): AccountAddress = newAccountAddressSampleStokenet()

    override fun other(): AccountAddress = newAccountAddressSampleStokenetOther()

    override fun  random(): AccountAddress = newAccountAddressRandom(networkId = NetworkId.STOKENET)
}

@VisibleForTesting
val AccountAddress.Companion.sampleStokenet: AccountAddressSampleStokenet
    get() = AccountAddressSampleStokenet

@VisibleForTesting
fun AccountAddress.Companion.sampleRandom(
    networkId: NetworkId
) = newAccountAddressRandom(networkId = networkId)