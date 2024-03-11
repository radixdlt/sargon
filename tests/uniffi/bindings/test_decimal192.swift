import Foundation
import Sargon

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
	public static func + (lhs: Self, rhs: Self) -> Self {
		decimalAdd(lhs: lhs, rhs: rhs)
	}
}
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

extension Decimal192 {
    public init(float: Float32) throws {
        self = try newDecimalFromF32(value: float)
    }
}

extension Decimal192: ExpressibleByFloatLiteral {
    public init(floatLiteral value: Float32) {
        do {
            try self.init(float: value)
        } catch {
            fatalError("Error: \(error)")
        }
    }
}

func test() throws {
	let one: Decimal192 = 1
	let two: Decimal192 = 2
	let three: Decimal192 = 3

	assert(one + two == three)

	var a: Decimal192 =
		"958947355801916604025588861116008628224.01234"

	var b: Decimal192 = "58947355801916604025588861116008628224.04321"
	var c: Decimal192 = "1017894711603833208051177722232017256448.05555"
	assert(a + b == c)

	a = 0.000000000000000123
	b = 0.000000000000000321
	c = 0.000000000000000444
	assert(String(describing: a) == "0.000000000000000123")
	assert(a + b == c)
}

try! test()
