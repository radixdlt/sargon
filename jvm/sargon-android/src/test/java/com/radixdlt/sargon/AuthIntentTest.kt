package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.hash
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class AuthIntentTest : SampleTestable<AuthIntent> {
    override val samples: List<Sample<AuthIntent>>
        get() = listOf(AuthIntent.sample)

    @Test
    fun testHash() {
        val authIntent = AuthIntent.sample()

        assertEquals(
            AuthIntentHash.sample(),
            authIntent.hash()
        )
    }

}