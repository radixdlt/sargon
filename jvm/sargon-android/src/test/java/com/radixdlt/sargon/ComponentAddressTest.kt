package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ComponentAddressTest: SampleTestable<ComponentAddress> {

    override val samples: List<Sample<ComponentAddress>>
        get() = listOf(ComponentAddress.sampleMainnet, ComponentAddress.sampleStokenet)

    @Test
    fun test() {
        val addressString = "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"
        val componentAddress = ComponentAddress.init(validatingAddress = addressString)

        assertEquals(addressString, componentAddress.string)
        assertEquals(NetworkId.MAINNET, componentAddress.networkId)
    }

}