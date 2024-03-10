package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.hexToBagOfBytes
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.toBagOfBytes
import com.radixdlt.sargon.sample.acedBagOfBytesSample
import com.radixdlt.sargon.sample.appendingCafeSample
import com.radixdlt.sargon.sample.appendingDeadbeefSample
import com.radixdlt.sargon.sample.babeBagOfBytesSample
import com.radixdlt.sargon.sample.cafeBagOfBytesSample
import com.radixdlt.sargon.sample.deadBagOfBytesSample
import com.radixdlt.sargon.sample.ecadBagOfBytesSample
import com.radixdlt.sargon.sample.fadeBagOfBytesSample
import com.radixdlt.sargon.sample.prependingCafeSample
import com.radixdlt.sargon.sample.prependingDeadbeefSample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class BagOfBytesTest {

    @Test
    fun test() {
        var a = ubyteArrayOf().toList()
        var b = ubyteArrayOf().toList()
        assert(a == b)

        a = ubyteArrayOf(129.toUByte()).toList()
        b = byteArrayOf(129.toByte()).toBagOfBytes()
        assert(a == b)

        assertEquals(
            acedBagOfBytesSample,
            "acedacedacedacedacedacedacedacedacedacedacedacedacedacedacedaced".hexToBagOfBytes()
        )
        assertEquals(
            babeBagOfBytesSample,
            "babebabebabebabebabebabebabebabebabebabebabebabebabebabebabebabe".hexToBagOfBytes()
        )
        assertEquals(
            cafeBagOfBytesSample,
            "cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe".hexToBagOfBytes()
        )
        assertEquals(
            deadBagOfBytesSample,
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead".hexToBagOfBytes()
        )
        assertEquals(
            ecadBagOfBytesSample,
            "ecadecadecadecadecadecadecadecadecadecadecadecadecadecadecadecad".hexToBagOfBytes()
        )
        assertEquals(
            fadeBagOfBytesSample,
            "fadefadefadefadefadefadefadefadefadefadefadefadefadefadefadefade".hexToBagOfBytes()
        )

        a = "beef".hexToBagOfBytes()
        assertEquals("beefcafe", a.appendingCafeSample().hex)
        assertEquals("beefdeadbeef", a.appendingDeadbeefSample().hex)
        assertEquals("cafebeef", a.prependingCafeSample().hex)
        assertEquals("deadbeefbeef", a.prependingDeadbeefSample().hex)

        b = "42".hexToBagOfBytes()
        assertEquals(
            "deadbeefcafe42cafedeadbeef",
            b.appendingCafeSample()
                .appendingDeadbeefSample()
                .prependingCafeSample()
                .prependingDeadbeefSample()
                .hex
        )

        (0.toUByte()..UByte.MAX_VALUE).forEach {
            val bytes = ubyteArrayOf(it.toUByte())
            assertEquals(bytes.toList(), bytes.toByteArray().toBagOfBytes())
        }

        val size = 100
        val set = (0..<size).map { randomBagOfBytes(byteCount = 16 + it) }.toSet()
        assertEquals(size, set.size)
    }

}