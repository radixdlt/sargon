package com.radixdlt.sargon.os.driver

import com.radixdlt.sargon.LogLevel
import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import timber.log.Timber

private typealias AndroidLogger = android.util.Log

class AndroidLoggingDriverTest {

    private val logTree = TestTree()

    @BeforeEach
    fun installLogger() {
        Timber.plant(logTree)
    }

    @AfterEach
    fun removeLogger() {
        Timber.uproot(logTree)
    }

    @Test
    fun `test logs emitted in correct level`() {
        val input = listOf(
            TestTree.Log(level = LogLevel.INFO, tag = "sargon", message = "info"),
            TestTree.Log(level = LogLevel.TRACE, tag = "sargon", message = "verbose"),
            TestTree.Log(level = LogLevel.WARN, tag = "sargon", message = "warn"),
            TestTree.Log(level = LogLevel.ERROR, tag = "sargon", message = "error"),
            TestTree.Log(level = LogLevel.DEBUG, tag = "sargon", message = "debug")
        )
        // Setting to false to avoid planting android debug tree, since logTree is planted.
        val sut = AndroidLoggingDriver(isLoggingEnabled = false)

        input.forEach { log ->
            sut.log(log.level, log.message)
        }

        assertEquals(
            input,
            logTree.logs
        )
    }

    private class TestTree: Timber.Tree() {

        private val _logs = mutableListOf<Log>()

        val logs: List<Log>
            get() = _logs

        override fun log(priority: Int, tag: String?, message: String, t: Throwable?) {
            val level = when (priority) {
                AndroidLogger.VERBOSE -> LogLevel.TRACE
                AndroidLogger.INFO -> LogLevel.INFO
                AndroidLogger.WARN -> LogLevel.WARN
                AndroidLogger.DEBUG -> LogLevel.DEBUG
                AndroidLogger.ERROR -> LogLevel.ERROR
                else -> null
            }

            if (level != null) {
                _logs.add(Log(level, tag, message))
            }
        }

        data class Log(
            val level: LogLevel,
            val tag: String? = null,
            val message: String
        )
    }


}