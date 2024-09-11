package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class ManifestEncounteredComponentAddressTest: SampleTestable<ManifestEncounteredComponentAddress> {

    override val samples: List<Sample<ManifestEncounteredComponentAddress>>
        get() = listOf(ManifestEncounteredComponentAddress.sampleMainnet, ManifestEncounteredComponentAddress.sampleStokenet)

    @Test
    fun testInit() {
        val componentAddress = "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"

        with(ManifestEncounteredComponentAddress.init(validating = componentAddress)) {
            assertEquals(componentAddress, string)
            assertEquals(NetworkId.MAINNET, networkId)
        }

        assertThrows<CommonException.FailedToDecodeAddressFromBech32> {
            ManifestEncounteredComponentAddress.init(validating = PackageAddress.sampleMainnet().string)
        }
    }

    @Test
    fun testFormat() {
        val addressString = "locker_rdx1dqeryv3jxgeryv3jxgeryv3jxgeryv3jxgeryv3jxgeryv3jjs0l6p"
        val address = ManifestEncounteredComponentAddress.init(validating = addressString)

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

}