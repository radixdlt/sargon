package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DepositRule
import com.radixdlt.sargon.depositRuleToJsonString
import com.radixdlt.sargon.newDepositRuleFromJsonString

@Throws(SargonException::class)
fun DepositRule.Companion.fromJson(jsonString: String): DepositRule =
    newDepositRuleFromJsonString(jsonString = jsonString)

fun DepositRule.toJson(): String = depositRuleToJsonString(depositRule = this)

