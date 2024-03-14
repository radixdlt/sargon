extension Decimal192 {
	public init(_ string: String) throws {
		self = try newDecimalFromString(string: string)
	}
}

extension Decimal192: CustomStringConvertible {
	public var description: String {
		decimalToString(decimal: self)
	}
}

extension Decimal192 {
	public static let maxDivisibility: UInt8 = 18
}

extension Decimal192 {
	/// Parse a local respecting string
	public init(
		formattedString: String,
		locale: Locale = .autoupdatingCurrent
	) throws {
		let localConfig: LocaleConfig = LocaleConfig(locale: locale)
		self = try newDecimalFromFormattedString(
			formattedString: formattedString,
			locale: localConfig
		)
	}
}

// MARK: Formatting
extension Option<UInt> {
	fileprivate var asU8: UInt8? {
		if let value = self {
		precondition(
			value < UInt(UInt8.max),
			"Must not specify more than \(UInt8.max) places, got: \(value).")
			value
		} else {
			nil
		}


	}
}
extension Decimal192 {

	/// A human readable, locale respecting string, rounded to `totalPlaces` places, counting all digits
	public func formatted(
		locale: Locale = .autoupdatingCurrent,
		totalPlaces: UInt? = nil,
		useGroupingSeparator: Bool = true
	) -> String {
		decimalFormatted(
			locale: LocaleConfig(locale: local),
			totalPlaces: totalPlaces.asU8,
			useGroupingSeparator: useGroupingSeparator
		)
	}

	public func formattedEngineeringNotation(
		locale: Locale = .autoupdatingCurrent,
		totalPlaces: UInt? = nil
	) -> String {
		decimalFormattedEngineeringNotation(
			locale: LocaleConfig(locale: local),
			totalPlaces: totalPlaces.asU8
		)
	}

	/// A human readable, locale respecting string. Does not perform any rounding or truncation.
	public func formattedPlain(
		locale: Locale = .autoupdatingCurrent,
		useGroupingSeparator: Bool = true
	) -> String {
		decimalFormattedPlain(
			locale: LocaleConfig(locale: local),
			useGroupingSeparator: useGroupingSeparator
		)
	}
}

// MARK: Truncation and rounding
extension Decimal192 {

	private func rounded(decimalPlaces: UInt8, roundingMode: RoundingMode) -> Self {
		precondition(
			decimalPlaces <= Decimal192.maxDivisibility,
			"Decimal places MUST be 0...18, was: \(decimalPlaces)"
		)
		do {
			return try decimalRound(
				decimal: self,
				decimalPlaces: Int32(decimalPlaces),
				roundingMode: roundingMode
			)
		} catch {
			fatalError("Failed to round, error: \(error)")
		}
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

	public func isNegative() -> Bool {
		decimalIsNegative(decimal: self)
	}
}

extension Decimal192: Comparable {
	public static func > (lhs: Self, rhs: Self) -> Bool {
		lhs.greaterThan(other: rhs)
	}
	public static func < (lhs: Self, rhs: Self) -> Bool {
		lhs.lessThan(other: rhs)
	}
	public static func >= (lhs: Self, rhs: Self) -> Bool {
		lhs.greaterThanOrEqual(other: rhs)
	}
	public static func <= (lhs: Self, rhs: Self) -> Bool {
		lhs.lessThanOrEqual(other: rhs)
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
