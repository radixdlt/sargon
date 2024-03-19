package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Decimal192
import com.radixdlt.sargon.LocaleConfig
import com.radixdlt.sargon.RoundingMode
import com.radixdlt.sargon.decimalAdd
import com.radixdlt.sargon.decimalClampedToZero
import com.radixdlt.sargon.decimalDiv
import com.radixdlt.sargon.decimalIsNegative
import com.radixdlt.sargon.decimalIsZero
import com.radixdlt.sargon.decimalMul
import com.radixdlt.sargon.decimalRound
import com.radixdlt.sargon.decimalSub
import com.radixdlt.sargon.decimalToString
import com.radixdlt.sargon.newDecimalFromF32
import com.radixdlt.sargon.newDecimalFromFormattedString
import com.radixdlt.sargon.newDecimalFromI32
import com.radixdlt.sargon.newDecimalFromI64
import com.radixdlt.sargon.newDecimalFromString
import com.radixdlt.sargon.newDecimalFromU32
import com.radixdlt.sargon.newDecimalFromU64
import java.text.DecimalFormatSymbols

fun String.toDecimal192() = newDecimalFromString(string = this)
fun Long.toDecimal192() = newDecimalFromI64(value = this)
fun Float.toDecimal192() = newDecimalFromF32(value = this)
fun Int.toDecimal192() = newDecimalFromI32(value = this)
fun ULong.toDecimal192() = newDecimalFromU64(value = this)
fun UInt.toDecimal192() = newDecimalFromU32(value = this)

val Decimal192.Companion.MAX_DIVISIBILITY: UByte
    get() = 18u.toUByte()

fun Decimal192.Companion.init(
    formattedString: String,
    decimalFormat: DecimalFormatSymbols = DecimalFormatSymbols.getInstance()
): Decimal192 {
    val config = LocaleConfig(
        decimalSeparator = decimalFormat.decimalSeparator.toString(),
        groupingSeparator = decimalFormat.groupingSeparator.toString()
    )

    return newDecimalFromFormattedString(
        formattedString = formattedString,
        locale = config
    )
}

val Decimal192.string: String
    get() = decimalToString(decimal = this)

val Decimal192.clamped: Decimal192
    get() = decimalClampedToZero(decimal = this)

val Decimal192.isNegative: Boolean
    get() = decimalIsNegative(decimal = this)

val Decimal192.isZero: Boolean
    get() = decimalIsZero(decimal = this)


fun Decimal192.rounded(decimalPlaces: UByte, roundingMode: RoundingMode): Decimal192 {
    require(decimalPlaces <= Decimal192.MAX_DIVISIBILITY) {
        "Decimal places MUST be 0...18, was: $decimalPlaces"
    }

    return try {
        decimalRound(decimal = this, decimalPlaces = decimalPlaces, roundingMode = roundingMode)
    } catch (exception: Exception) {
        error("Failed to round, error: $exception")
    }
}

/**
 * Rounds to [decimalPlaces] decimals
 */
fun Decimal192.rounded(decimalPlaces: UByte = 0u.toUByte()): Decimal192 = rounded(
    decimalPlaces = decimalPlaces,
    roundingMode = RoundingMode.TO_NEAREST_MIDPOINT_AWAY_FROM_ZERO
)

/**
 * Rounds to [decimalPlaces] decimals, in the direction of 0
 */
fun Decimal192.floor(decimalPlaces: UByte): Decimal192 = rounded(
    decimalPlaces = decimalPlaces,
    roundingMode = RoundingMode.TO_ZERO
)

/**
 * Rounds to [decimalPlaces] decimals, in the direction away of 0
 */
fun Decimal192.ceil(decimalPlaces: UByte): Decimal192 = rounded(
    decimalPlaces = decimalPlaces,
    roundingMode = RoundingMode.AWAY_FROM_ZERO
)


operator fun Decimal192.plus(other: Decimal192): Decimal192 = decimalAdd(lhs = this, rhs = other)

operator fun Decimal192.minus(other: Decimal192): Decimal192 = decimalSub(lhs = this, rhs = other)

operator fun Decimal192.times(other: Decimal192): Decimal192 = decimalMul(lhs = this, rhs = other)

operator fun Decimal192.div(other: Decimal192): Decimal192 = decimalDiv(lhs = this, rhs = other)

operator fun Decimal192.compareTo(other: Decimal192): Int {
    val diff = this - other
    return when {
        diff.isNegative -> -1
        diff.isZero -> 0
        else -> 1
    }
}
