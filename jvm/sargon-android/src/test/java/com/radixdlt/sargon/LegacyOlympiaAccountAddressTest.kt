package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.isLegacyOfBabylonAddress
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.toBabylonAddress
import com.radixdlt.sargon.extensions.wasMigratedFromLegacyOlympia
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class LegacyOlympiaAccountAddressTest: SampleTestable<LegacyOlympiaAccountAddress> {

    override val samples: List<Sample<LegacyOlympiaAccountAddress>>
        get() = listOf(LegacyOlympiaAccountAddress.sample)

    @Test
    fun testInit() {
        val address = LegacyOlympiaAccountAddress.sample()
        assertEquals(
            address,
            LegacyOlympiaAccountAddress.init(validatingAddress = address.string)
        )

        val publicKey = PublicKey.Secp256k1.init("026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c")
        assertEquals(
            "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842",
            LegacyOlympiaAccountAddress.init(publicKey).string
        )
    }

    @Test
    fun testMigrated() {
        val babylon: AccountAddress = AccountAddress.init("account_rdx168e8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdh0pwrf")
        val legacy = LegacyOlympiaAccountAddress.sample()
        assertTrue(legacy.isLegacyOfBabylonAddress(babylon))
        assertTrue(babylon.wasMigratedFromLegacyOlympia(legacy))
        assertEquals(
            babylon,
            legacy.toBabylonAddress()
        )
        assertEquals(
            NetworkId.MAINNET,
            legacy.networkId
        )
    }

}