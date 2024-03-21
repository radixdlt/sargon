extension Decimal192: @unchecked Sendable {}
extension Decimal192: SargonModel {}

#if DEBUG
extension Decimal192: ExpressibleByStringLiteral {
	public init(stringLiteral string: String) {
		try! self.init(string)
	}
}

extension Decimal192: ExpressibleByFloatLiteral {
	public init(floatLiteral float: Float32) {
		try! self.init(float)
	}
}
#endif

extension Decimal192: ExpressibleByIntegerLiteral {
	public init(integerLiteral i64: Int64) {
		self = Self(i64)
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

extension Decimal192: AdditiveArithmetic {
	public static func + (lhs: Self, rhs: Self) -> Self {
		lhs.add(rhs: rhs)
	}
	public static func - (lhs: Self, rhs: Self) -> Self {
		lhs.sub(rhs: rhs)
	}
}

extension Decimal192: SignedNumeric {
	public prefix static func - (operand: Self) -> Self {
		operand.negate()
	}
}

extension Decimal192: Numeric {
	public typealias Magnitude = Self

	public var magnitude: Magnitude {
		abs()
	}

	public static func * (lhs: Self, rhs: Self) -> Self {
		lhs.mul(rhs: rhs)
	}

	public static func *= (lhs: inout Self, rhs: Self) {
		lhs = lhs * rhs
	}

	public init?<T>(exactly source: T) where T: BinaryInteger {
		if let i64 = Int64(exactly: source) {
			self = Self(i64)
		} else if let u64 = UInt64(exactly: source) {
			self = Self(u64)
		} else {
			return nil
		}
	}
}

extension Decimal192 {
	public static func / (lhs: Self, rhs: Self) -> Self {
		lhs.div(rhs: rhs)
	}
}
