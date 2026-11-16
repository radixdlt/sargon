package com.radixdlt.sargon.os.storage

import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions.assertInstanceOf
import org.junit.jupiter.api.Test

class KeystoreAccessRequestTest {

    @Test
    fun testSpecsOfProfile() = runTest {
        val request = KeystoreAccessRequest.ForProfile

        assertInstanceOf(KeySpec.Profile::class.java, request.keySpec)

        try {
            request.requestAuthorization().getOrThrow()
        } catch (exception: Exception) {
            assert(false) { "requestAuthorization for Profile should succeed but didn't" }
        }
    }

    @Test
    fun testSpecsOfRadixConnect() = runTest {
        val request = KeystoreAccessRequest.ForRadixConnect

        assertInstanceOf(KeySpec.RadixConnect::class.java, request.keySpec)

        try {
            request.requestAuthorization().getOrThrow()
        } catch (exception: Exception) {
            assert(false) { "requestAuthorization for Radix Connect should succeed but didn't" }
        }
    }

    @Test
    fun testSpecsOfCache() = runTest {
        val request = KeystoreAccessRequest.ForCache("an alias")

        assertInstanceOf(KeySpec.Cache::class.java, request.keySpec)

        try {
            request.requestAuthorization().getOrThrow()
        } catch (exception: Exception) {
            assert(false) { "requestAuthorization for Radix Connect should succeed but didn't" }
        }
    }

}
