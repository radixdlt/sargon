package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.SecureStorageAccessErrorKind
import com.radixdlt.sargon.secureStorageAccessErrorKindIsManualCancellation

fun SecureStorageAccessErrorKind.isManualCancellation() =
    secureStorageAccessErrorKindIsManualCancellation(kind = this)