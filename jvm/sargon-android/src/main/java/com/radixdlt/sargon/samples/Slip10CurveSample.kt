package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Slip10Curve
import com.radixdlt.sargon.annotation.UsesSampleValues

@UsesSampleValues
val Slip10Curve.Companion.sample: Sample<Slip10Curve>
    get() = object : Sample<Slip10Curve> {
        override fun invoke(): Slip10Curve = Slip10Curve.CURVE25519

        override fun other(): Slip10Curve = Slip10Curve.SECP256K1
    }