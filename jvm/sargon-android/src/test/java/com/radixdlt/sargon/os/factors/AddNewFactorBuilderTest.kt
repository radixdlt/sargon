package com.radixdlt.sargon.os.factors

import com.radixdlt.sargon.Bios
import com.radixdlt.sargon.Drivers
import com.radixdlt.sargon.FactorSourceKind
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
import io.mockk.mockk
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.runTest
import okhttp3.OkHttpClient
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class AddNewFactorBuilderTest {

    private val okHttpClient = mockk<OkHttpClient>()
    private val eventBusDriver = AndroidEventBusDriver
    private val profileStateChangeDriver = AndroidProfileStateChangeDriver
    private val hostInteractor = FakeHostInteractor()

    private val testDispatcher = StandardTestDispatcher()

    @Test
    fun addTest() = runTest(testDispatcher) {
        val sargonOs = SargonOs.boot(bios(), hostInteractor)
        var builder = sargonOs.makeAddNewFactorBuilder(FactorSourceKind.DEVICE)
        builder = builder.setName("New Test Factor")
        assertEquals("New Test Factor", builder.getName())
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
            profileStateChangeDriver = profileStateChangeDriver
        )
    )
}