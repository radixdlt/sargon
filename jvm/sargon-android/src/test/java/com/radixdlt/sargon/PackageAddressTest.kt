package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class PackageAddressTest: SampleTestable<PackageAddress> {

    override val samples: List<Sample<PackageAddress>>
        get() = listOf(PackageAddress.sampleMainnet, PackageAddress.sampleStokenet)

    @Test
    fun test() {
        val addressString = "package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet"
        val packageAddress = PackageAddress.init(validatingAddress = addressString)

        assertEquals(addressString, packageAddress.string)
        assertEquals(NetworkId.MAINNET, packageAddress.networkId)
    }

}