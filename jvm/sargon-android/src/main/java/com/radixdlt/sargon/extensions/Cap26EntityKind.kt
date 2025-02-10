package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Cap26EntityKind
import com.radixdlt.sargon.cap26EntityKindToString

val Cap26EntityKind.discriminant: String
    get() = cap26EntityKindToString(kind = this)