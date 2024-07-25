package com.radixdlt.sargon.os

import com.radixdlt.sargon.HostOs
import com.radixdlt.sargon.SampleTestable
import com.radixdlt.sargon.extensions.android
import com.radixdlt.sargon.extensions.name
import com.radixdlt.sargon.extensions.other
import com.radixdlt.sargon.extensions.vendor
import com.radixdlt.sargon.extensions.version
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class HostOsTest: SampleTestable<HostOs> {
    override val samples: List<Sample<HostOs>>
        get() = listOf(HostOs.sample)

    @Test
    fun testGetName() {
        val sut = HostOs.android("Google", "14 (API 34)")

        assertEquals("Android", sut.name)
    }

    @Test
    fun testGetVendor() {
        val sut = HostOs.android("Google", "14 (API 34)")

        assertEquals("Google", sut.vendor)
    }

    @Test
    fun testGetVersion() {
        val sut = HostOs.android("Google", "14 (API 34)")

        assertEquals("Android 14 (API 34)", sut.version)
    }

    @Test
    fun testOther() {
        val sut = HostOs.other("macos", "Apple", "14.5")

        assertEquals("macos", sut.name)
        assertEquals("Apple", sut.vendor)
        assertEquals("macos 14.5", sut.version)
    }
}