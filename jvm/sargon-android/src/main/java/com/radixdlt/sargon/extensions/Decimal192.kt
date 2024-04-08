package com.radixdlt.sargon.extensions
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.Decimal192
import com.radixdlt.sargon.LocaleConfig
import com.radixdlt.sargon.RoundingMode
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.decimalAbs
import com.radixdlt.sargon.decimalAdd
import com.radixdlt.sargon.decimalClampedToZero
import com.radixdlt.sargon.decimalDiv
import com.radixdlt.sargon.decimalFormatted
import com.radixdlt.sargon.decimalFormattedPlain
import com.radixdlt.sargon.decimalIsNegative
import com.radixdlt.sargon.decimalIsPositive
import com.radixdlt.sargon.decimalIsZero
import com.radixdlt.sargon.decimalMax
import com.radixdlt.sargon.decimalMin
import com.radixdlt.sargon.decimalMul
import com.radixdlt.sargon.decimalNeg
import com.radixdlt.sargon.decimalRound
import com.radixdlt.sargon.decimalSub
import com.radixdlt.sargon.decimalToString
import com.radixdlt.sargon.newDecimalExponent
import com.radixdlt.sargon.newDecimalFromF32
import com.radixdlt.sargon.newDecimalFromF64
import com.radixdlt.sargon.newDecimalFromFormattedString
import com.radixdlt.sargon.newDecimalFromI32
import com.radixdlt.sargon.newDecimalFromI64
import com.radixdlt.sargon.newDecimalFromString
import com.radixdlt.sargon.newDecimalFromU32
import com.radixdlt.sargon.newDecimalFromU64
import java.text.DecimalFormatSymbols

/**
 * Tries to creates a new [Decimal192] from a String, throws a `CommonError`
 * if the `string` was not a valid Decimal192.
 */
@Throws(SargonException::class)
fun String.toDecimal192() = newDecimalFromString(string = this)
fun String.toDecimal192OrNull() = runCatching { toDecimal192() }.getOrNull()

/**
 * Creates a new [Decimal192] from a i64 integer.
 */
fun Long.toDecimal192() = newDecimalFromI64(value = this)

/**
 * Creates a new [Decimal192] from a [Float]. Will
 * fail if the [Float] cannot be losslessly represented
 * by the underlying Decimal from Scrypto.
 */
@Throws(SargonException::class)
fun Float.toDecimal192() = newDecimalFromF32(value = this)
fun Float.toDecimal192OrNull() = runCatching { toDecimal192() }.getOrNull()

/**
 * Creates a new [Decimal192] from a [Double]. Will
 * fail if the [Double] cannot be losslessly represented
 * by the underlying Decimal from Scrypto.
 */
@Throws(SargonException::class)
fun Double.toDecimal192() = newDecimalFromF64(value = this)
fun Double.toDecimal192OrNull() = runCatching { toDecimal192() }.getOrNull()

/**
 * Creates a new [Decimal192] from a i32 integer.
 */
fun Int.toDecimal192() = newDecimalFromI32(value = this)

/**
 * Creates a new [Decimal192] from a u64 integer.
 */
fun ULong.toDecimal192() = newDecimalFromU64(value = this)

/**
 * Creates a new [Decimal192] from a u32 integer.
 */
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

/**
 * The maximum possible value of [Decimal192], being:
 * `3138550867693340381917894711603833208051.177722232017256447`
 */
val Decimal192.Companion.MAX: Decimal192
    get() = decimalMax()

/**
 * The minimum possible value of [Decimal192], being:
 * `-3138550867693340381917894711603833208051.177722232017256448`
 */
val Decimal192.Companion.MIN: Decimal192
    get() = decimalMin()

/**
 * Creates the [Decimal192] `10^exponent`
 */
fun Decimal192.Companion.exponent(exponent: UByte): Decimal192 =
    newDecimalExponent(exponent = exponent)

val Decimal192.string: String
    get() = decimalToString(decimal = this)

/**
 * Clamps `decimal` to zero, i.e. `max(decimal, 0)`
 */
val Decimal192.clamped: Decimal192
    get() = decimalClampedToZero(decimal = this)

/**
 * Whether this decimal is negative.
 */
val Decimal192.isNegative: Boolean
    get() = decimalIsNegative(decimal = this)

/**
 * Whether this decimal is positive.
 */
val Decimal192.isPositive: Boolean
    get() = decimalIsPositive(decimal = this)

/**
 * Whether this decimal is zero.
 */
val Decimal192.isZero: Boolean
    get() = decimalIsZero(decimal = this)


/**
 * Rounds this number to the specified decimal places.
 *
 * @throws CommonException if the number of decimal places is not within [0..SCALE(=18)]
 */
@Throws(SargonException::class)
fun Decimal192.rounded(decimalPlaces: UByte, roundingMode: RoundingMode): Decimal192 {
    require(decimalPlaces <= Decimal192.MAX_DIVISIBILITY) {
        "Decimal places MUST be 0...18, was: $decimalPlaces"
    }

    return try {
        decimalRound(
            decimal = this,
            decimalPlaces = decimalPlaces.toUByte(),
            roundingMode = roundingMode
        )
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

/**
 * Returns `decimal.abs()`, throws if `decimal` is [Decimal192.Companion.MIN]
 */
fun Decimal192.abs(): Decimal192 = decimalAbs(decimal = this)

/**
 * Negates the `decimal`
 */
fun Decimal192.negative(): Decimal192 = decimalNeg(decimal = this)

fun Decimal192?.orZero() = this ?: 0.toDecimal192()

@KoverIgnore
fun Decimal192.formatted(
    format: DecimalFormatSymbols = DecimalFormatSymbols.getInstance(),
    totalPlaces: UByte = 8u,
    useGroupingSeparator: Boolean = true
) = decimalFormatted(
    decimal = this,
    locale = format.toLocaleConfig(),
    totalPlaces = totalPlaces,
    useGroupingSeparator = useGroupingSeparator
)

/**
 * A human readable, locale respecting string. Does not perform any rounding or truncation.
 */
@KoverIgnore
fun Decimal192.formattedPlain(
    format: DecimalFormatSymbols = DecimalFormatSymbols.getInstance(),
    useGroupingSeparator: Boolean = true
) = decimalFormattedPlain(
    decimal = this,
    locale = format.toLocaleConfig(),
    useGroupingSeparator = useGroupingSeparator
)

inline fun <T> Iterable<T>.sumOf(selector: (T) -> Decimal192): Decimal192 {
    var sum: Decimal192 = 0.toDecimal192()
    for (element in this) {
        sum += selector(element)
    }
    return sum
}

private fun DecimalFormatSymbols.toLocaleConfig() = LocaleConfig(
    decimalSeparator = decimalSeparator.toString(),
    groupingSeparator = groupingSeparator.toString()
)