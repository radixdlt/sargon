package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Subintent
import com.radixdlt.sargon.SubintentHash
import com.radixdlt.sargon.subintentCompile
import com.radixdlt.sargon.subintentHash

fun Subintent.hash(): SubintentHash = subintentHash(intent = this)

fun Subintent.compile(): BagOfBytes = subintentCompile(intent = this)