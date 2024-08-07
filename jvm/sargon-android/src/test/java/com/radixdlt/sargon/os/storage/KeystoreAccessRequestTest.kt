package com.radixdlt.sargon.os.storage

import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Assertions.assertDoesNotThrow
import org.junit.jupiter.api.Assertions.assertInstanceOf
import org.junit.jupiter.api.Assertions.assertTrue
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
    fun testSpecsOfMnemonic() = runTest {
        val request = KeystoreAccessRequest.ForMnemonic(
            onRequestAuthorization = { Result.success(Unit) }
        )
        assertInstanceOf(KeySpec.Mnemonic::class.java, request.keySpec)

        try {
            request.requestAuthorization().getOrThrow()
        } catch (exception: Exception) {
            assert(false) { "requestAuthorization for Mnemonic should succeed but didn't" }
        }

        val failingRequest = KeystoreAccessRequest.ForMnemonic(
            onRequestAuthorization = { Result.failure(RuntimeException("An error")) }
        )

        try {
            failingRequest.requestAuthorization().getOrThrow()
            assert(false) { "requestAuthorization for failing access to Mnemonic should fail but succeeded" }
        } catch (exception: Exception) {
            assert(true) { "requestAuthorization for failing access to Mnemonic should fail" }
        }
    }

}