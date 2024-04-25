package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Bip39WordCount
import com.radixdlt.sargon.CommonException

@Throws(SargonException::class)
fun Bip39WordCount.Companion.init(wordCount: Int) = Bip39WordCount.entries.find {
    it.value == wordCount.toUByte()
} ?: throw CommonException.InvalidBip39WordCount(badValue = wordCount.toULong())
