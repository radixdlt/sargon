package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.RequestedNumberQuantifier
import com.radixdlt.sargon.RequestedQuantity
import com.radixdlt.sargon.newRequestedQuantityFromJsonBytes
import com.radixdlt.sargon.requestedQuantityIsFulfilledByIds
import com.radixdlt.sargon.requestedQuantityIsValid
import com.radixdlt.sargon.requestedQuantityToJsonBytes

val RequestedQuantity.isValid: Boolean
    get() = requestedQuantityIsValid(requestedQuantity = this)

fun RequestedQuantity.isFulfilled(ids: Int): Boolean = requestedQuantityIsFulfilledByIds(
    requestedQuantity = this,
    numberOfIds = ids.toULong()
)

fun RequestedQuantity.Companion.exactly(quantity: Int): RequestedQuantity =
    RequestedQuantity(
        quantifier = RequestedNumberQuantifier.EXACTLY,
        quantity = quantity.toUShort()
    )

fun RequestedQuantity.Companion.atLeast(quantity: Int): RequestedQuantity =
    RequestedQuantity(
        quantifier = RequestedNumberQuantifier.AT_LEAST,
        quantity = quantity.toUShort()
    )

fun RequestedQuantity.Companion.deserializeFromJsonString(json: String) =
    newRequestedQuantityFromJsonBytes(jsonBytes = bagOfBytes(fromString = json))

fun RequestedQuantity.serializedJsonString() =
    requestedQuantityToJsonBytes(requestedQuantity = this).string

