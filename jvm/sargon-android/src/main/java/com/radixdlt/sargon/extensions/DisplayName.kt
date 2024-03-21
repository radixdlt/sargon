package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DisplayName
import com.radixdlt.sargon.newDisplayName

@Throws(SargonException::class)
fun DisplayName.Companion.init(validating: String) =
    newDisplayName(name = validating)