package com.radixdlt.sargon.os

import app.cash.turbine.test
import com.radixdlt.sargon.Bios
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.Drivers
import com.radixdlt.sargon.SargonOs
import com.radixdlt.sargon.os.driver.AndroidEntropyProviderDriver
import com.radixdlt.sargon.os.driver.AndroidEventBusDriver
import com.radixdlt.sargon.os.driver.AndroidNetworkingDriver
import com.radixdlt.sargon.os.driver.AndroidProfileStateChangeDriver
import com.radixdlt.sargon.os.driver.FakeFileSystemDriver
import com.radixdlt.sargon.os.driver.FakeHostInfoDriver
import com.radixdlt.sargon.os.driver.FakeLoggingDriver
import com.radixdlt.sargon.os.driver.FakeSecureStorageDriver
import com.radixdlt.sargon.os.driver.FakeUnsafeStorageDriver
import com.radixdlt.sargon.os.interactor.FakeHostInteractor
import io.mockk.coEvery
import io.mockk.mockk
import io.mockk.mockkObject
import io.mockk.unmockkObject
import kotlinx.coroutines.Job
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestScope
import kotlinx.coroutines.test.runTest
import okhttp3.OkHttpClient
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertInstanceOf
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import com.radixdlt.sargon.NfcTagDriverImpl
import com.radixdlt.sargon.ArculusCsdkDriverImpl
import com.radixdlt.sargon.NoPointer


class SargonOsManagerTest {
    private val okHttpClient = mockk<OkHttpClient>()
    private val eventBusDriver = AndroidEventBusDriver
    private val profileStateChangeDriver = AndroidProfileStateChangeDriver
    private val hostInteractor = FakeHostInteractor()

    private val testDispatcher = StandardTestDispatcher()
    private val testScope = TestScope(testDispatcher + Job())

    @Test
    fun testBoot() = runTest(testDispatcher) {
        val manager = SargonOsManager.factory(
            bios = bios(),
            hostInteractor = hostInteractor,
            applicationScope = testScope,
            defaultDispatcher = testDispatcher
        )

        manager.sargonState.test {
            assert(awaitItem() is SargonOsState.Idle)
            assertThrows<SargonOsNotBooted> {
                manager.sargonOs
            }

            assert(awaitItem() is SargonOsState.Booted)
            assertInstanceOf(SargonOs::class.java, manager.sargonOs)
        }

        val newManager = SargonOsManager.factory(
            bios = bios(),
            hostInteractor = hostInteractor,
            applicationScope = testScope,
            defaultDispatcher = testDispatcher
        )
        assertEquals(newManager, manager)

        SargonOsManager.tearDown()
    }

    @Test
    fun testBootFails() = runTest(testDispatcher) {
        val bios = bios()

        mockkObject(SargonOs)
        coEvery {
            SargonOs.boot(bios, hostInteractor)
        } throws(CommonException.Unknown())

        val manager = SargonOsManager.factory(
            bios = bios,
            hostInteractor = hostInteractor,
            applicationScope = testScope,
            defaultDispatcher = testDispatcher
        )

        manager.sargonState.test {
            assert(awaitItem() is SargonOsState.Idle)
            assertThrows<SargonOsNotBooted> {
                manager.sargonOs
            }

            val state = awaitItem() as SargonOsState.BootError
            assert(state.error is CommonException.Unknown)
            assertThrows<SargonOsNotBooted> {
                manager.sargonOs
            }
        }
        unmockkObject(SargonOs)

        SargonOsManager.tearDown()
    }

    private fun bios() = Bios(
        drivers = Drivers(
            networking = AndroidNetworkingDriver(client = okHttpClient),
            secureStorage = FakeSecureStorageDriver(),
            unsafeStorage = FakeUnsafeStorageDriver(),
            entropyProvider = AndroidEntropyProviderDriver(),
            hostInfo = FakeHostInfoDriver(),
            logging = FakeLoggingDriver(),
            eventBus = eventBusDriver,
            fileSystem = FakeFileSystemDriver(),
            profileStateChangeDriver = profileStateChangeDriver,
            arculusCsdkDriver = ArculusCsdkDriverImpl(NoPointer),
            nfcTagDriver = NfcTagDriverImpl(NoPointer)
        )
    )
}