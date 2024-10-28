package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.NonFungibleLocalId
import com.radixdlt.sargon.newNonFungibleLocalIdBytes
import com.radixdlt.sargon.newNonFungibleLocalIdFromString
import com.radixdlt.sargon.newNonFungibleLocalIdInt
import com.radixdlt.sargon.newNonFungibleLocalIdRuid
import com.radixdlt.sargon.newNonFungibleLocalIdString

/**
 * Creates a child type of [NonFungibleLocalId] from the local id string
 *
 * Expects local id format like: `#1#` for [NonFungibleLocalId.Integer] type,
 * or `<foo>` for [NonFungibleLocalId.Str] type, etc...
 *
 * @throws SargonException if the [localId] is invalid string.
 */
@Throws(SargonException::class)
fun NonFungibleLocalId.Companion.init(localId: String): NonFungibleLocalId =
    newNonFungibleLocalIdFromString(localId = localId)

/**
 * Creates a [NonFungibleLocalId.Bytes] id
 *
 * For example creates an id from:
 * "deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210" => `Bytes([deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210]`)
 * @throws SargonException if the [bytes] are invalid
 */
@Throws(SargonException::class)
fun NonFungibleLocalId.Companion.bytesId(bytes: BagOfBytes): NonFungibleLocalId =
    newNonFungibleLocalIdBytes(bytes = bytes)

/**
 * Creates a [NonFungibleLocalId.Integer] id
 *
 * For example creates an id from: "1" => `Integer(#1#)`
 */
fun NonFungibleLocalId.Companion.intId(value: ULong): NonFungibleLocalId =
    newNonFungibleLocalIdInt(value = value)

/**
 * Creates a [NonFungibleLocalId.Ruid] id
 *
 * For example creates an id from:
 * "deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210" => `Ruid({deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210})`
 * @throws SargonException if the [value] bytes are invalid
 */
@Throws(SargonException::class)
fun NonFungibleLocalId.Companion.ruidId(value: BagOfBytes): NonFungibleLocalId =
    newNonFungibleLocalIdRuid(bytes = value)

/**
 * Creates a [NonFungibleLocalId.Str] id. Not to be confused with
 * [NonFungibleLocalId.Companion.init]
 *
 * For example creates an id from: "foo" => `Str(<foo>)`
 *
 * @throws SargonException if the [value] bytes are invalid
 */
@Throws(SargonException::class)
fun NonFungibleLocalId.Companion.stringId(string: String): NonFungibleLocalId =
    newNonFungibleLocalIdString(string = string)

val NonFungibleLocalId.string: String
    get() = this.asString

fun NonFungibleLocalId.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = this.formatted.getString(format)