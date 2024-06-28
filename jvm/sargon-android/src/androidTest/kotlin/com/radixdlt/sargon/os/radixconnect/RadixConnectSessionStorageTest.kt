package com.radixdlt.sargon.os.radixconnect

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.filters.SmallTest
import androidx.test.platform.app.InstrumentationRegistry
import com.radixdlt.sargon.SessionId
import com.radixdlt.sargon.extensions.randomBagOfBytes
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
@SmallTest
class RadixConnectSessionStorageTest {

    private val sut = RadixConnectSessionStorage(
        context = InstrumentationRegistry.getInstrumentation().context
    )

    @Test
    fun testRoundtrip() = runTest(context = StandardTestDispatcher()) {
        val sessionId = SessionId.randomUUID()
        val sessionBytes = randomBagOfBytes(32)

        assertNull(sut.loadSession(sessionId))
        sut.saveSession(sessionId, sessionBytes)
        assertEquals(sessionBytes, sut.loadSession(sessionId))
    }

}