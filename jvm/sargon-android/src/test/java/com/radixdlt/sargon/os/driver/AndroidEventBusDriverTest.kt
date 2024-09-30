package com.radixdlt.sargon.os.driver

import app.cash.turbine.test
import app.cash.turbine.turbineScope
import com.radixdlt.sargon.Event
import com.radixdlt.sargon.EventNotification
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.Timestamp
import com.radixdlt.sargon.Uuid
import com.radixdlt.sargon.samples.sample
import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class AndroidEventBusDriverTest {

    private val sut = AndroidEventBusDriver

    @Test
    fun testProfileIsEmitted() = runTest {
        val event = EventNotification(
            id = Uuid.randomUUID(),
            event = Event.Booted,
            timestamp = Timestamp.now()
        )

        sut.events.test {
            // First subscribe to event changes (this is a shared flow) then emit a value
            sut.handleEventNotification(event)

            // Then assert values are received
            assertEquals(event, awaitItem())
        }
    }

    @Test
    fun testMulticast() = runTest {
        val event = EventNotification(
            id = Uuid.randomUUID(),
            event = Event.Booted,
            timestamp = Timestamp.now()
        )

        turbineScope {
            val firstSubscriber = sut.events.testIn(backgroundScope)
            val secondSubscriber = sut.events.testIn(backgroundScope)

            sut.handleEventNotification(event)

            assertEquals(event, firstSubscriber.awaitItem())
            assertEquals(event, secondSubscriber.awaitItem())
        }
    }

}