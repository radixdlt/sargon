package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.TransactionManifest
import com.radixdlt.sargon.transactionManifestToString

val TransactionManifest.string: String
    get() = transactionManifestToString(manifest = this)