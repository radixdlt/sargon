package com.radixdlt.sargon.os.driver

import com.radixdlt.sargon.LogLevel
import com.radixdlt.sargon.LoggingDriver
import timber.log.Timber

class AndroidLoggingDriver(
    isLoggingEnabled: Boolean
): LoggingDriver {

    init {
        if (isLoggingEnabled) {
            Timber.plant(Timber.DebugTree())
        }
    }

    override fun log(level: LogLevel, msg: String) {
        val logger = Timber.tag("sargon")
        when (level) {
            LogLevel.ERROR -> logger.e(msg)
            LogLevel.WARN -> logger.w(msg)
            LogLevel.INFO -> logger.i(msg)
            LogLevel.DEBUG -> logger.d(msg)
            LogLevel.TRACE -> logger.v(msg)
        }
    }
}