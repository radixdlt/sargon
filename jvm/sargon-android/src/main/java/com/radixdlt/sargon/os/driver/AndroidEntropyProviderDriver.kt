package com.radixdlt.sargon.os.driver

import com.radixdlt.sargon.Entropy32Bytes
import com.radixdlt.sargon.EntropyProviderDriver
import com.radixdlt.sargon.extensions.random

class AndroidEntropyProviderDriver: EntropyProviderDriver {
    override fun generateSecureRandomBytes(): Entropy32Bytes = Entropy32Bytes.random()
}