package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ValidatorAddressTest {

    @Test
    fun test() {
        val addressString = "validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0"
        val validatorAddress = ValidatorAddress.init(validatingAddress = addressString)

        assertEquals(addressString, validatorAddress.string)
        assertEquals(NetworkId.MAINNET, validatorAddress.networkId)
    }

}