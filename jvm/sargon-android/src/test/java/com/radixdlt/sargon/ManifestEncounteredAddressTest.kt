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

class ManifestEncounteredAddressTest: SampleTestable<ManifestEncounteredAddress> {

    override val samples: List<Sample<ManifestEncounteredAddress>>
        get() = listOf(ManifestEncounteredAddress.sampleMainnet, ManifestEncounteredAddress.sampleStokenet)

    @Test
    fun testInit() {
        val componentAddress = "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"

        with(ManifestEncounteredAddress.init(validating = componentAddress)) {
            assertEquals(componentAddress, string)
            assertEquals(NetworkId.MAINNET, networkId)
        }

        assertThrows<CommonException.FailedToDecodeAddressFromBech32> {
            ManifestEncounteredAddress.init(validating = PackageAddress.sampleMainnet().string)
        }
    }

    @Test
    fun testFormat() {
        val addressString = "locker_rdx1dqeryv3jxgeryv3jxgeryv3jxgeryv3jxgeryv3jxgeryv3jjs0l6p"
        val address = ManifestEncounteredAddress.init(validating = addressString)

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