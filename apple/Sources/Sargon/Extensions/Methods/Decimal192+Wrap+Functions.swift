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
