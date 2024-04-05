package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.errorCodeFromError
import com.radixdlt.sargon.errorMessageFromError

typealias SargonException = CommonException

val SargonException.errorMessage: String
    get() = errorMessageFromError(error = this)

val SargonException.errorCode: UInt
    get() = errorCodeFromError(error = this)