package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.signatureWithPublicKey
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class IntentSignatureTest: SampleTestable<IntentSignature> {
    override val samples: List<Sample<IntentSignature>>
        get() = listOf(IntentSignature.sample)

    @Test
    fun test_get_signature_with_public_key() {
        val signature = SignatureWithPublicKey.sample()
        assertEquals(
            signature,
            IntentSignature.init(signatureWithPublicKey = signature).signatureWithPublicKey
        )
    }

}