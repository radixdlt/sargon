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
	public static let maxDivisibility: UInt = 18
}

// MARK: Truncation and rounding

extension Decimal192 {
	/// Rounds to `decimalPlaces` decimals, in the direction of 0
	public func floor(decimalPlaces: UInt) -> Self {
		try! round(decimalPlaces: Int32(decimalPlaces), roundingMode: .toZero)
	}

	/// Rounds to `decimalPlaces` decimals, in the direction away from zero
	public func ceil(decimalPlaces: UInt) -> Self {
		try! round(decimalPlaces: Int32(decimalPlaces), roundingMode: .awayFromZero)
	}

	/// Rounds to `decimalPlaces` decimals
	public func rounded(decimalPlaces: UInt = 0) -> Self {
		try! round(decimalPlaces: Int32(decimalPlaces), roundingMode: .toNearestMidpointAwayFromZero)
	}
}

extension Decimal192 {
	public var clamped: Self {
		isNegative() ? .zero : self
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
