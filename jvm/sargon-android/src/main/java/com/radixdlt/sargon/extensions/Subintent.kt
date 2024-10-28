package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.CompiledSubintent
import com.radixdlt.sargon.Subintent
import com.radixdlt.sargon.SubintentHash
import com.radixdlt.sargon.subintentCompile
import com.radixdlt.sargon.subintentHash

fun Subintent.hash(): SubintentHash = subintentHash(subintent = this)

fun Subintent.compile(): CompiledSubintent = subintentCompile(subintent = this)