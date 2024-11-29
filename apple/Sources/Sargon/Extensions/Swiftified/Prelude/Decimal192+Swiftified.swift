import Foundation
import SargonUniFFI

// MARK: - Decimal192 + SargonModel
extension Decimal192: SargonModel {}

// MARK: - Decimal192 + ExpressibleByIntegerLiteral
extension Decimal192: ExpressibleByIntegerLiteral {
	public init(integerLiteral i64: Int64) {
		self = Self(i64)
	}
}

// MARK: - Decimal192 + Comparable
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

// MARK: - Decimal192 + AdditiveArithmetic
extension Decimal192: AdditiveArithmetic {
	public static func + (lhs: Self, rhs: Self) -> Self {
		lhs.add(rhs: rhs)
	}

	public static func - (lhs: Self, rhs: Self) -> Self {
		lhs.sub(rhs: rhs)
	}
}

// MARK: - Decimal192 + SignedNumeric
extension Decimal192: SignedNumeric {
	public static prefix func - (operand: Self) -> Self {
		operand.negate()
	}
}

// MARK: - Decimal192 + Numeric
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

	public init?(exactly source: some BinaryInteger) {
		if let u64 = UInt64(exactly: source) {
			self = Self(u64)
		} else if let i64 = Int64(exactly: source) {
			self = Self(i64)
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

extension Decimal192 {
	public var asDouble: Double {
		// this can never fail
		Double(self.toRawString())!
	}
}

extension Decimal192 {
	public func toRawString() -> String {
		decimalToString(decimal: self)
	}
}

// MARK: - Decimal192 + Codable
extension Decimal192: Codable {
	@inlinable
	public func encode(to encoder: Encoder) throws {
		var container = encoder.singleValueContainer()
		try container.encode(toRawString())
	}

	@inlinable
	public init(from decoder: Decoder) throws {
		let container = try decoder.singleValueContainer()
		let string = try container.decode(String.self)
		try self.init(string)
	}
}

extension Decimal192 {
	public static let one: Self = 1
	public static let two: Self = 2
	public static let three: Self = 3
	public static let four: Self = 4
	public static let five: Self = 5
	public static let six: Self = 6
	public static let seven: Self = 7
	public static let eight: Self = 8
	public static let nine: Self = 9
	public static let ten: Self = 10
}
