package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.UnsafeStorageKey
import com.radixdlt.sargon.unsafeStorageKeyIdentifier

val UnsafeStorageKey.identifier: String
    get() = unsafeStorageKeyIdentifier(key = this)