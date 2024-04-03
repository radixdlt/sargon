package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class AddressTest {

    @Test
    fun testAddressFromVariousTypes() {
        with(AccessControllerAddress.sampleMainnet()) {
            assertEquals(
                Address.AccessController(this),
                Address.init(string)
            )
        }
        with(AccountAddress.sampleMainnet()) {
            assertEquals(
                Address.Account(this),
                Address.init(string)
            )
        }
        with(ComponentAddress.sampleMainnet()) {
            assertEquals(
                Address.Component(this),
                Address.init(string)
            )
        }
        with(IdentityAddress.sampleMainnet()) {
            assertEquals(
                Address.Identity(this),
                Address.init(string)
            )
        }
        with(PackageAddress.sampleMainnet()) {
            assertEquals(
                Address.Package(this),
                Address.init(string)
            )
        }
        with(PoolAddress.sampleMainnet()) {
            assertEquals(
                Address.Pool(this),
                Address.init(string)
            )
        }
        with(PoolAddress.sampleMainnet()) {
            assertEquals(
                Address.Pool(this),
                Address.init(string)
            )
        }
        with(ResourceAddress.sampleMainnet()) {
            assertEquals(
                Address.Resource(this),
                Address.init(string)
            )
        }
        with(ValidatorAddress.sampleMainnet()) {
            assertEquals(
                Address.Validator(this),
                Address.init(string)
            )
        }
        with(VaultAddress.sampleMainnet()) {
            assertEquals(
                Address.Vault(this),
                Address.init(string)
            )
        }
    }

    @Test
    fun testFormatted() {
        val sample = AccountAddress.sampleMainnet()
        val address = Address.Account(sample)

        assertEquals(sample.formatted(), address.formatted())
        assertEquals(
            sample.formatted(format = AddressFormat.DEFAULT),
            address.formatted(format = AddressFormat.DEFAULT)
        )
        assertEquals(
            sample.formatted(format = AddressFormat.RAW),
            address.formatted(format = AddressFormat.RAW)
        )
        assertEquals(
            sample.formatted(format = AddressFormat.FULL),
            address.formatted(format = AddressFormat.FULL)
        )
    }

    @Test
    fun testNetworkId() {
        val sample = AccountAddress.sampleMainnet()
        val address = Address.Account(sample)

        assertEquals(
            sample.networkId,
            address.networkId
        )
    }

    @Test
    fun testString() {
        val sample = AccountAddress.sampleMainnet()
        val address = Address.Account(sample)

        assertEquals(
            sample.string,
            address.string
        )
    }
}