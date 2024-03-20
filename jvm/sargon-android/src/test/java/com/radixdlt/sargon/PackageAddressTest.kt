package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class PackageAddressTest {

    @Test
    fun test() {
        val addressString = "package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet"
        val packageAddress = PackageAddress.init(validatingAddress = addressString)

        assertEquals(addressString, packageAddress.string)
        assertEquals(NetworkId.MAINNET, packageAddress.networkId)
    }

}