package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Account
import com.radixdlt.sargon.Persona
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.constantEntityNameMaxLength
import com.radixdlt.sargon.constantMinRequiredXrdForAccountDeletion

@KoverIgnore
object SharedConstants {
    val minRequiredXrdForAccountDeletion = constantMinRequiredXrdForAccountDeletion()
}

@KoverIgnore
val Account.Companion.nameMaxLength: Long
    get() = constantEntityNameMaxLength().toLong()

@KoverIgnore
val Persona.Companion.nameMaxLength: Long
    get() = constantEntityNameMaxLength().toLong()