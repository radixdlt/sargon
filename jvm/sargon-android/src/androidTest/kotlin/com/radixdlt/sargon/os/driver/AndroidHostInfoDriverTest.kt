package com.radixdlt.sargon.os.driver

import android.content.Context
import android.content.pm.PackageInfo
import android.content.pm.PackageManager
import android.os.Build
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.filters.SmallTest
import androidx.test.platform.app.InstrumentationRegistry
import com.radixdlt.sargon.HostOs
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
@SmallTest
class AndroidHostInfoDriverTest {

    val sut = AndroidHostInfoDriver(
        context = InstrumentationRegistry.getInstrumentation().context
    )

    @Suppress("DEPRECATION")
    @Test
    fun test_host_os() = runTest {
        val hostOs = sut.hostOs() as HostOs.Android

        assertEquals(
            Build.MANUFACTURER.capitalize(),
            hostOs.vendor
        )
        assertEquals(
            "${Build.VERSION.RELEASE} (API ${Build.VERSION.SDK_INT})",
            hostOs.version
        )
    }

    @Test
    fun test_app_version_instrumentation() = runTest {
        // App version when this library is tested in a standalone manner should be empty
        assertEquals(
            "",
            sut.hostAppVersion()
        )
    }

    @Test
    fun test_app_version_in_app() = runTest {
        val testPackage = "com.sargon.android.test"
        val testVersion = "1.0.0"
        val packageManager = mockk<PackageManager>().apply {
            every { getPackageInfo(testPackage, 0) } returns PackageInfo().apply {
                versionName = testVersion
            }
        }
        val applicationContext = mockk<Context>().apply {
            every { packageName } returns testPackage
            every { this@apply.packageManager } returns packageManager
        }
        val context = mockk<Context>().apply {
            every { this@apply.applicationContext } returns applicationContext
        }


        val sut = AndroidHostInfoDriver(context = context)

        assertEquals(
            testVersion,
            sut.hostAppVersion()
        )
    }
}