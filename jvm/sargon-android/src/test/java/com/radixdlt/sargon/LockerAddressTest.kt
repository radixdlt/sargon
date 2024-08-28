package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class LockerAddressTest: SampleTestable<LockerAddress> {

    override val samples: List<Sample<LockerAddress>>
        get() = listOf(LockerAddress.sampleMainnet, LockerAddress.sampleStokenet)

    @Test
    fun test() {
        val addressString = "locker_rdx1dqeryv3jxgeryv3jxgeryv3jxgeryv3jxgeryv3jxgeryv3jjs0l6p"
        val lockerAddress = LockerAddress.init(validatingAddress = addressString)

        assertEquals(addressString, lockerAddress.string)
        assertEquals(NetworkId.MAINNET, lockerAddress.networkId)
    }

    @Test
    fun testFormat() {
        val addressString = "locker_rdx1dqeryv3jxgeryv3jxgeryv3jxgeryv3jxgeryv3jxgeryv3jjs0l6p"
        val address = LockerAddress.init(validatingAddress = addressString)

        assertEquals("lock...js0l6p", address.formatted())
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
        val address = LockerAddress.sampleMainnet()

        assertEquals(Address.Locker(address), address.asGeneral())
    }
}