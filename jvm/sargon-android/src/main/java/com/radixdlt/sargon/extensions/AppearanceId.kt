package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AppearanceId
import com.radixdlt.sargon.appearanceIdsAll
import com.radixdlt.sargon.newAppearanceId

@Throws(SargonException::class)
fun AppearanceId.Companion.init(validating: UByte) = newAppearanceId(validating = validating)

fun AppearanceId.Companion.all() = appearanceIdsAll()