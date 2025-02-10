package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Account
import com.radixdlt.sargon.DisplayName
import com.radixdlt.sargon.Persona
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.constantDisplayNameMaxLength
import com.radixdlt.sargon.constantEntityNameMaxLength
import com.radixdlt.sargon.constantMinRequiredXrdForAccountDeletion
import com.radixdlt.sargon.extensions.SharedConstants.displayNameMaxLength
import com.radixdlt.sargon.extensions.SharedConstants.entityNameMaxLength

@KoverIgnore
object SharedConstants {
    val minRequiredXrdForAccountDeletion = constantMinRequiredXrdForAccountDeletion()
    val entityNameMaxLength = constantEntityNameMaxLength().toLong()
    val displayNameMaxLength = constantDisplayNameMaxLength().toLong()
}

@KoverIgnore
val Account.Companion.nameMaxLength: Long
    @KoverIgnore
    get() = entityNameMaxLength

@KoverIgnore
val Persona.Companion.nameMaxLength: Long
    @KoverIgnore
    get() = entityNameMaxLength

@KoverIgnore
val DisplayName.Companion.maxLength: Long
    @KoverIgnore
    get() = displayNameMaxLength