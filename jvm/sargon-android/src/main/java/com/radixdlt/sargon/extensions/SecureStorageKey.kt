package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.secureStorageKeyIdentifier

val SecureStorageKey.identifier: String
    get() = secureStorageKeyIdentifier(key = this)