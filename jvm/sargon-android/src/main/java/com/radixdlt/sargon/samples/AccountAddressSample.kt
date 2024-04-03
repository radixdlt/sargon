package com.radixdlt.sargon.samples

import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newAccountAddressRandom
import com.radixdlt.sargon.newAccountAddressSampleMainnet
import com.radixdlt.sargon.newAccountAddressSampleMainnetOther
import com.radixdlt.sargon.newAccountAddressSampleStokenet
import com.radixdlt.sargon.newAccountAddressSampleStokenetOther

@UsesSampleValues
object AccountAddressSampleMainnet: SampleWithRandomValues<AccountAddress> {
    override fun invoke(): AccountAddress = newAccountAddressSampleMainnet()

    override fun other(): AccountAddress = newAccountAddressSampleMainnetOther()

    override fun random(): AccountAddress = newAccountAddressRandom(networkId = NetworkId.MAINNET)
}

@UsesSampleValues
val AccountAddress.Companion.sampleMainnet: AccountAddressSampleMainnet
    get() = AccountAddressSampleMainnet

@UsesSampleValues
object AccountAddressSampleStokenet: SampleWithRandomValues<AccountAddress> {
    override fun invoke(): AccountAddress = newAccountAddressSampleStokenet()

    override fun other(): AccountAddress = newAccountAddressSampleStokenetOther()

    override fun  random(): AccountAddress = newAccountAddressRandom(networkId = NetworkId.STOKENET)
}

@UsesSampleValues
val AccountAddress.Companion.sampleStokenet: AccountAddressSampleStokenet
    get() = AccountAddressSampleStokenet

@UsesSampleValues
fun AccountAddress.Companion.sampleRandom(
    networkId: NetworkId
) = newAccountAddressRandom(networkId = networkId)