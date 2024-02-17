extension Decimal192: Sendable {}

extension Decimal192: ExpressibleByStringLiteral {
	public init(stringLiteral string: String) {
		try! self.init(string)
	}
}
extension Decimal192: ExpressibleByIntegerLiteral {
	public init(integerLiteral i64: Int64) {
		self = newDecimalFromI64(value: i64)
	}
}


extension Decimal192: AdditiveArithmetic {
	public static var zero: Self {
		newDecimalFromU64(value: 0)
	}
	public static func + (lhs: Self, rhs: Self) -> Self {
		decimalAdd(lhs: lhs, rhs: rhs)
	}
	public static func - (lhs: Self, rhs: Self) -> Self {
		decimalSub(lhs: lhs, rhs: rhs)
	}
}
extension Decimal192: SignedNumeric {
	public prefix static func - (operand: Self) -> Self {
		decimalNeg(decimal: operand)
	}
}
extension Decimal192: Numeric {
	public typealias Magnitude = Self

	public var magnitude: Magnitude {
		decimalAbs(decimal: self)
	}

	public static func * (lhs: Self, rhs: Self) -> Self {
		decimalMul(lhs: lhs, rhs: rhs)
	}

	public static func *= (lhs: inout Self, rhs: Self) {
		lhs = lhs * rhs
	}

	public init?<T>(exactly source: T) where T: BinaryInteger {
		if let i64 = Int64(exactly: source) {
			self = newDecimalFromI64(value: i64)
		} else if let u64 = UInt64(exactly: source) {
			self = newDecimalFromU64(value: u64)
		} else {
			return nil
		}
	}
}
