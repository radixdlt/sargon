package com.radixdlt.sargon.os.driver

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.filters.SmallTest
import androidx.test.platform.app.InstrumentationRegistry
import com.radixdlt.sargon.extensions.bagOfBytes
import com.radixdlt.sargon.extensions.string
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestScope
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import timber.log.Timber
import java.io.File

@RunWith(AndroidJUnit4::class)
@SmallTest
class AndroidFileSystemDriverTest {

    @Test
    fun given_no_file_when_save_invoked_then_file_is_created() = runTest {
        val sut = sut()

        sut.saveToFile(TEMP_FILE_NAME, bagOfBytes(PAYLOAD))

        val retrievedData = sut.loadFromFile(TEMP_FILE_NAME)
        assertEquals(
            PAYLOAD,
            retrievedData?.string
        )
    }

    @Test
    fun given_file_exists_when_save_invoked_then_file_contents_are_replaced() = runTest {
        val sut = sut()

        sut.saveToFile(TEMP_FILE_NAME, bagOfBytes(PAYLOAD))
        sut.saveToFile(TEMP_FILE_NAME, bagOfBytes("Michael"))

        val retrievedData = sut.loadFromFile(TEMP_FILE_NAME)
        assertEquals(
            "Michael",
            retrievedData?.string
        )
    }

    @Test
    fun given_file_exists_when_delete_invoked_then_file_deleted() = runTest {
        val sut = sut()

        sut.saveToFile(TEMP_FILE_NAME, bagOfBytes(PAYLOAD))
        // Ensure file exists prior to delete
        assertTrue(
            File(
                sargonDir,
                TEMP_FILE_NAME
            ).exists()
        )

        sut.deleteFile(TEMP_FILE_NAME)

        assertFalse(
            File(
                sargonDir,
                TEMP_FILE_NAME
            ).exists()
        )
    }

    @Test
    fun given_no_file_exists_when_delete_invoked_then_file_remains_non_existent() = runTest {
        val sut = sut()

        // Ensure file does not exist prior to delete
        assertFalse(
            File(
                sargonDir,
                TEMP_FILE_NAME
            ).exists()
        )

        sut.deleteFile(TEMP_FILE_NAME)

        assertFalse(
            File(
                sargonDir,
                TEMP_FILE_NAME
            ).exists()
        )
    }

    @Test
    fun given_file_deleted_when_read_invoked_then_null_returned() = runTest {
        val sut = sut()
        sut.saveToFile(TEMP_FILE_NAME, bagOfBytes(PAYLOAD))
        // Ensure file exists prior to delete
        assertTrue(
            File(
                sargonDir,
                TEMP_FILE_NAME
            ).exists()
        )

        sut.deleteFile(TEMP_FILE_NAME)

        assertNull(
            sut.loadFromFile(TEMP_FILE_NAME)
        )
    }

    @After
    fun clean() {
        sargonDir.deleteRecursively()
    }

    private fun TestScope.sut() = AndroidFileSystemDriver(
        context = InstrumentationRegistry.getInstrumentation().context,
        dispatcher = StandardTestDispatcher(testScheduler)
    ).also {
        Timber.plant(Timber.DebugTree())
    }

    companion object {
        private val sargonDir = File(
            InstrumentationRegistry.getInstrumentation().context.filesDir,
            AndroidFileSystemDriver.BASE_DIR
        )

        private const val TEMP_FILE_NAME = "file.txt"
        private const val PAYLOAD = "The quick brown fox jumps over the lazy dog"
    }
}