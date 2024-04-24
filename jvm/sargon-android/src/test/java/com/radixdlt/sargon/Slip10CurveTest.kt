package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Slip10CurveTest {

    @Test
    fun testRoundtrip() {
        val curve25519 = Slip10Curve.CURVE25519

        assertEquals(curve25519, Slip10Curve.init(curve25519.string))


        val secp256k1 = Slip10Curve.SECP256K1

        assertEquals(secp256k1, Slip10Curve.init(secp256k1.string))
    }

    @Test
    fun testJsonRoundtrip() {
        val sut = Slip10Curve.sample()
        assertEquals(
            sut,
            Slip10Curve.fromJson(sut.toJson())
        )
    }

}