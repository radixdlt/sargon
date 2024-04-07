package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.poolKind
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class PoolAddressTest: SampleTestable<PoolAddress> {

    override val samples: List<Sample<PoolAddress>>
        get() = listOf(PoolAddress.sampleMainnet, PoolAddress.sampleStokenet)

    @Test
    fun test() {
        val singlePoolAddressString = "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"
        val twoPoolAddressString = "pool_rdx1c5dkfdtdqvczcwzdyvzeuhddyha768p2q28erden533fty8h68ay6m"
        val multiPoolAddressString = "pool_rdx1cc7etecr23e77z9aqvq9rg43ndh9jkt7dzmaygg4t8c36z8qe6k47t"

        with(PoolAddress.init(validatingAddress = singlePoolAddressString)) {
            assertEquals(singlePoolAddressString, string)
            assertEquals(NetworkId.MAINNET, networkId)
            assertEquals(PoolKind.ONE_RESOURCE, poolKind)
        }

        with(PoolAddress.init(validatingAddress = twoPoolAddressString)) {
            assertEquals(twoPoolAddressString, string)
            assertEquals(NetworkId.MAINNET, networkId)
            assertEquals(PoolKind.TWO_RESOURCES, poolKind)
        }

        with(PoolAddress.init(validatingAddress = multiPoolAddressString)) {
            assertEquals(multiPoolAddressString, string)
            assertEquals(NetworkId.MAINNET, networkId)
            assertEquals(PoolKind.MULTI_RESOURCES, poolKind)
        }
    }

    @Test
    fun testFormat() {
        val addressString = "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"
        val address = PoolAddress.init(validatingAddress = addressString)

        assertEquals("pool...p3le3w", address.formatted())
        assertEquals(
            addressString,
            address.formatted(format = AddressFormat.FULL)
        )
        assertEquals(
            addressString,
            address.formatted(format = AddressFormat.RAW)
        )
    }

    @Test
    fun testAsGeneral() {
        val address = PoolAddress.sampleMainnet()

        assertEquals(Address.Pool(address), address.asGeneral())
    }

}