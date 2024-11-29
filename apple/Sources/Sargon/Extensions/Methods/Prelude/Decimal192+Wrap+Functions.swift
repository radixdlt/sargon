import Foundation
import SargonUniFFI

#if DEBUG
import XCTestDynamicOverlay
#endif // DEBUG

extension Decimal192 {
	public init(_ string: String) throws {
		self = try newDecimalFromString(string: string)
	}

	public init(_ float32: Float32) {
		self = newDecimalFromF32(value: float32)
	}

	public init(_ double: Double) throws {
		self = try newDecimalFromF64(value: double)
	}

	public init(_ value: Int64) {
		self = newDecimalFromI64(value: value)
	}

	public init(_ value: UInt64) {
		self = newDecimalFromU64(value: value)
	}

	/// Creates the Decimal `10^exponent`
	public init(exponent: UInt8) {
		self = newDecimalExponent(exponent: exponent)
	}
}

#if DEBUG
extension Locale {
	public static let test = Self(identifier: "en_US_POSIX")
}
#endif // DEBUG

// MARK: - Decimal192 + CustomStringConvertible
extension Decimal192: CustomStringConvertible {
	public var description: String {
		#if DEBUG
		if !_XCTIsTesting {
			formattedPlain()
		} else {
			formattedPlain(locale: .test, useGroupingSeparator: false)
		}
		#else
		formattedPlain()
		#endif // DEBUG
	}
}

// MARK: - Decimal192 + CustomDebugStringConvertible
extension Decimal192: CustomDebugStringConvertible {
	public var debugDescription: String {
		#if DEBUG
		if !_XCTIsTesting {
			formattedPlain()
		} else {
			formattedPlain(locale: .test, useGroupingSeparator: false)
		}
		#else
		formattedPlain()
		#endif // DEBUG
	}
}

extension Decimal192 {
	public static let maxDivisibility: UInt8 = 18

	public static let temporaryStandardFee: Self = transactionFeePreset()
}

extension Decimal192 {
	/// Parse a local respecting string
	public init(
		formattedString: String,
		locale: Locale = .autoupdatingCurrent
	) throws {
		let localConfig = LocaleConfig(locale: locale)
		self = try newDecimalFromFormattedString(
			formattedString: formattedString,
			locale: localConfig
		)
	}
}

// MARK: Formatting
extension Decimal192 {
	/// A human readable, locale respecting string, rounded to `totalPlaces` places, counting all digits
	public func formatted(
		locale: Locale = .autoupdatingCurrent,
		totalPlaces: UInt8 = 8,
		useGroupingSeparator: Bool = true
	) -> String {
		decimalFormatted(
			decimal: self,
			locale: LocaleConfig(locale: locale),
			totalPlaces: totalPlaces,
			useGroupingSeparator: useGroupingSeparator
		)
	}

	/// A human readable, locale respecting string. Does not perform any rounding or truncation.
	public func formattedPlain(
		locale: Locale = .autoupdatingCurrent,
		useGroupingSeparator: Bool = true
	) -> String {
		decimalFormattedPlain(
			decimal: self,
			locale: LocaleConfig(locale: locale),
			useGroupingSeparator: useGroupingSeparator
		)
	}
}

// MARK: Truncation and rounding
extension Decimal192 {
	private func rounded(decimalPlaces: UInt8, roundingMode: RoundingMode) -> Self {
		try! decimalRound(
			decimal: self,
			decimalPlaces: decimalPlaces,
			roundingMode: roundingMode
		)
	}

	/// Rounds to `decimalPlaces` decimals
	public func rounded(decimalPlaces: UInt8 = 0) -> Self {
		rounded(
			decimalPlaces: decimalPlaces,
			roundingMode: .toNearestMidpointAwayFromZero
		)
	}

	/// Rounds to `decimalPlaces` decimals, in the direction of 0
	public func floor(decimalPlaces: UInt8) -> Self {
		rounded(decimalPlaces: decimalPlaces, roundingMode: .toZero)
	}

	/// Rounds to `decimalPlaces` decimals, in the direction away from zero
	public func ceil(decimalPlaces: UInt8) -> Self {
		rounded(decimalPlaces: decimalPlaces, roundingMode: .awayFromZero)
	}
}

extension Decimal192 {
	public var clamped: Self {
		decimalClampedToZero(decimal: self)
	}

	public var isNegative: Bool {
		decimalIsNegative(decimal: self)
	}

	public var isPositive: Bool {
		decimalIsPositive(decimal: self)
	}

	public var isZero: Bool {
		decimalIsZero(decimal: self)
	}
}

extension Decimal192 {
	public func lessThan(other: Self) -> Bool {
		decimalLessThan(lhs: self, rhs: other)
	}

	public func lessThanOrEqual(other: Self) -> Bool {
		decimalLessThanOrEqual(lhs: self, rhs: other)
	}

	public func greaterThan(other: Self) -> Bool {
		decimalGreaterThan(lhs: self, rhs: other)
	}

	public func greaterThanOrEqual(other: Self) -> Bool {
		decimalGreaterThanOrEqual(lhs: self, rhs: other)
	}
}

extension Decimal192 {
	public static var zero: Self {
		newDecimalFromU64(value: 0)
	}

	public func add(rhs: Self) -> Self {
		decimalAdd(lhs: self, rhs: rhs)
	}

	public func sub(rhs: Self) -> Self {
		decimalSub(lhs: self, rhs: rhs)
	}

	public func mul(rhs: Self) -> Self {
		decimalMul(lhs: self, rhs: rhs)
	}

	public func div(rhs: Self) -> Self {
		decimalDiv(lhs: self, rhs: rhs)
	}

	public func abs() -> Self {
		decimalAbs(decimal: self)
	}

	public func negate() -> Self {
		decimalNeg(decimal: self)
	}
}

extension Decimal192 {
	/// Positive value
	public static var max: Self {
		decimalMax()
	}

	/// Negative value
	public static var min: Self {
		decimalMin()
	}
}
