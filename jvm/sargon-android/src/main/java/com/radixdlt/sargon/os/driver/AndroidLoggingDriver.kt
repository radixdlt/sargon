package com.radixdlt.sargon.os.driver

import com.radixdlt.sargon.LogLevel
import com.radixdlt.sargon.LoggingDriver
import timber.log.Timber

class AndroidLoggingDriver: LoggingDriver {
    override fun log(level: LogLevel, msg: String) {
        when (level) {
            LogLevel.ERROR -> Timber.e(msg)
            LogLevel.WARN -> Timber.w(msg)
            LogLevel.INFO -> Timber.i(msg)
            LogLevel.DEBUG -> Timber.d(msg)
            LogLevel.TRACE -> Timber.v(msg)
        }
    }
}