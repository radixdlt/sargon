package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Message
import com.radixdlt.sargon.messageAsPlaintext
import com.radixdlt.sargon.newMessagePlaintextString

fun Message.Companion.plaintext(string: String): Message = newMessagePlaintextString(
    string = string
)

val Message.plaintext: String?
    get() = messageAsPlaintext(message = this)