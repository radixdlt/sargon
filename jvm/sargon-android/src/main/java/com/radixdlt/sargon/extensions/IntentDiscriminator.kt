package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.IntentDiscriminator
import com.radixdlt.sargon.intentDiscriminatorGetValue
import com.radixdlt.sargon.newIntentDiscriminatorRandom

fun IntentDiscriminator.Companion.random() = newIntentDiscriminatorRandom()

val IntentDiscriminator.value: ULong
    get() = intentDiscriminatorGetValue(intentDiscriminator = this)