package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.toHttpMethod
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class NetworkMethodTest: SampleTestable<NetworkMethod> {

    @Test
    fun test() {
        assertEquals(
            "POST",
            NetworkMethod.POST.toHttpMethod()
        )
        assertEquals(
            "GET",
            NetworkMethod.GET.toHttpMethod()
        )
        assertEquals(
            "HEAD",
            NetworkMethod.HEAD.toHttpMethod()
        )
    }

    override val samples: List<Sample<NetworkMethod>>
        get() = listOf(NetworkMethod.sample)
}