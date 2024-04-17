package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.append
import com.radixdlt.sargon.extensions.contains
import com.radixdlt.sargon.extensions.get
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.remove
import com.radixdlt.sargon.extensions.size
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

class SupportedCurvesTest {

    @Test
    fun testListMethods() {
        val sample = Slip10Curve.CURVE25519
        val sampleOther = Slip10Curve.SECP256K1

        var list = SupportedCurves.init(sample)

        Assertions.assertTrue(sample in list)
        Assertions.assertEquals(1, list.size)
        Assertions.assertEquals(sample, list[0])

        list = list.append(sampleOther)
        Assertions.assertTrue(sampleOther in list)
        Assertions.assertEquals(2, list.size)
        Assertions.assertEquals(sampleOther, list[1])

        list = list.remove(sampleOther)
        Assertions.assertFalse(sampleOther in list)
        Assertions.assertEquals(1, list.size)
    }
}