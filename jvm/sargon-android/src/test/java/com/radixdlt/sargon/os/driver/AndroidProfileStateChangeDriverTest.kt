package com.radixdlt.sargon.os.driver

import app.cash.turbine.test
import app.cash.turbine.turbineScope
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.ProfileState
import com.radixdlt.sargon.samples.sample
import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class AndroidProfileStateChangeDriverTest {

    private val sut = AndroidProfileStateChangeDriver

    @Test
    fun testProfileIsEmitted() = runTest {
        val profile = Profile.sample()

        sut.profile.test {
            // First subscribe to profile changes (this is a shared flow) then emit a value
            sut.handleProfileStateChange(ProfileState.Loaded(profile))

            // Then assert values are received
            assertEquals(profile, awaitItem())
        }
    }

    @Test
    fun testMulticast() = runTest {
        val profile = Profile.sample()

        turbineScope {
            val firstSubscriber = sut.profile.testIn(backgroundScope)
            val secondSubscriber = sut.profile.testIn(backgroundScope)

            sut.handleProfileStateChange(ProfileState.Loaded(profile))

            assertEquals(profile, firstSubscriber.awaitItem())
            assertEquals(profile, secondSubscriber.awaitItem())
        }
    }

}