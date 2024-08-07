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

class ValidatorAddressTest: SampleTestable<ValidatorAddress> {

    override val samples: List<Sample<ValidatorAddress>>
        get() = listOf(ValidatorAddress.sampleMainnet, ValidatorAddress.sampleStokenet)

    @Test
    fun test() {
        val addressString = "validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0"
        val validatorAddress = ValidatorAddress.init(validatingAddress = addressString)

        assertEquals(addressString, validatorAddress.string)
        assertEquals(NetworkId.MAINNET, validatorAddress.networkId)
    }

    @Test
    fun testFormat() {
        val addressString = "validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0"
        val address = ValidatorAddress.init(validatingAddress = addressString)

        assertEquals("vali...pdeyy0", address.formatted())
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
        val address = ValidatorAddress.sampleMainnet()

        assertEquals(Address.Validator(address), address.asGeneral())
    }
}