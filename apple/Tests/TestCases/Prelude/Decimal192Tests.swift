#if DEBUG
extension Decimal192: ExpressibleByStringLiteral {
	public init(stringLiteral string: String) {
		try! self.init(string)
	}
}

extension Decimal192: ExpressibleByFloatLiteral {
	public init(floatLiteral float: Float32) {
		self.init(float)
	}
}
#endif

extension Decimal192 {
	public static let pi: Self = "3.141592653589793238"
	public static let e: Self = "2.718281828459045235"
}

extension Decimal192 {
	/// Positive unique values, sorted (increasing in size), excluding `.max`
	public static let positive: [Self] = {
		let values: [Self] = [
			Self.one,
			Self.two,
			Self.e,
			Self.three,
			Self.pi,
			Self.four,
			Self.five,
			Self.six,
			Self.seven,
			Self.eight,
			Self.nine,
			Self.ten,
			Self(21_000_000),
		]
		assert(values.sorted() == values)
		assert(Set(values).count == values.count) // assert no duplicates
		return values
	}()

	/// Sorted in increasing order: [-10, -9, .. -2, -1]
	public static let negative: [Self] = positive.map { $0.negate() }.sorted()

	public static let nonZero: [Self] = {
		var nonZero: [Self] = []
		nonZero.append(contentsOf: Self.negative)
		nonZero.append(contentsOf: Self.positive)
		return nonZero.sorted()
	}()
}

extension Array {
	var identityPairs: [(Element, Element)] {
		zip(self, self).map { ($0, $1) }
	}

	/// [2, 5, 7, 13].slidingWindowPairs == [(2, 5), (5, 7), (7, 13)]
	var slidingWindowPairs: [(Element, Element)] {
		enumerated().compactMap { offset, element in
			let nextIndex = offset + 1
			if nextIndex >= count {
				return nil
			}
			let nextElement = self[nextIndex]
			return (element, nextElement)
		}
	}
}

import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

// MARK: - Decimal192Tests
final class Decimal192Tests: Test<Decimal192> {
	func test_init_from_string() throws {
		let s = "3.1415"
		try XCTAssertNoDifference(SUT(s).debugDescription, s)
	}

	func test_init_from_i64() {
		let value: Int64 = -1337
		XCTAssertNoDifference(SUT(value).debugDescription, value.description)
	}

	func test_init_from_u64() {
		let value: UInt64 = 237
		XCTAssertNoDifference(SUT(value).debugDescription, value.description)
	}

	func test_max_divisibility() {
		XCTAssertEqual(SUT.maxDivisibility, 18)
	}

	func test_equal() {
		for (lhs, rhs) in SUT.positive.identityPairs {
			XCTAssertEqual(lhs, rhs)
		}
		for (lhs, rhs) in SUT.negative.identityPairs {
			XCTAssertEqual(lhs, rhs)
		}

		XCTAssertEqual(SUT.zero, SUT.zero)
		XCTAssertEqual(SUT.max, SUT.max)
		XCTAssertEqual(SUT.min, SUT.min)
		XCTAssertEqual(SUT.one, SUT.one)
		XCTAssertEqual(SUT.two, SUT.two)
		XCTAssertEqual(SUT.pi, SUT.pi)
		XCTAssertEqual(SUT.e, SUT.e)
	}

	func test_not_equal() {
		XCTAssertNotEqual(SUT.zero, SUT.max)
		XCTAssertNotEqual(SUT.zero, SUT.min)
		XCTAssertNotEqual(SUT.max, SUT.min)
		XCTAssertNotEqual(SUT.one, SUT.one.negate())
		XCTAssertNotEqual(SUT.pi, SUT.e)
	}

	func test_greater_than() {
		XCTAssertGreaterThan(SUT.one, SUT.zero)
		XCTAssertGreaterThan(SUT.max, SUT.ten)
		XCTAssertGreaterThan(SUT.max, SUT.min)
		XCTAssertGreaterThan(SUT.zero, SUT.min)
		XCTAssertGreaterThan(SUT.pi, SUT.e)
	}

	func test_greater_than_or_equal() {
		for (lhs, rhs) in SUT.nonZero.identityPairs {
			XCTAssertGreaterThanOrEqual(lhs, rhs)
		}
		for (lhs, rhs) in SUT.nonZero.slidingWindowPairs {
			XCTAssertGreaterThanOrEqual(rhs, lhs)
		}
		XCTAssertGreaterThanOrEqual(SUT.four, SUT.three)
		XCTAssertGreaterThanOrEqual(SUT.five, SUT.five)
		XCTAssertGreaterThanOrEqual(SUT.pi, SUT.e)
	}

	func test_less_than() {
		for (lhs, rhs) in SUT.nonZero.slidingWindowPairs {
			XCTAssertLessThan(lhs, rhs)
		}
		XCTAssertLessThan(SUT.nine, SUT.ten)
		XCTAssertLessThan(SUT.min, SUT.zero)
		XCTAssertLessThan(SUT.zero, SUT.max)
		XCTAssertLessThan(SUT.e, SUT.pi)
	}

	func test_less_than_or_equal() {
		for (lhs, rhs) in SUT.nonZero.identityPairs {
			XCTAssertLessThanOrEqual(lhs, rhs)
		}
		XCTAssertLessThanOrEqual(SUT.seven, SUT.eight)
		XCTAssertLessThanOrEqual(SUT.six, SUT.six)
		XCTAssertLessThanOrEqual(SUT.e, SUT.pi)
	}

	func test_addition() {
		for (lhs, rhs) in SUT.nonZero.slidingWindowPairs {
			XCTAssertEqual(lhs + rhs, rhs + lhs) // commutative
		}
		for item in SUT.negative {
			XCTAssertEqual(item + item, 2 * item)
		}

		for item in SUT.nonZero {
			// zero is identity under addition
			XCTAssertEqual(item + SUT.zero, item)
		}

		XCTAssertEqual(SUT.zero + 0, 0)
		XCTAssertEqual(SUT.zero + 1, 1)
		XCTAssertEqual(SUT.one + 1, 2)
		XCTAssertEqual(SUT.one + 2, 3)
		XCTAssertEqual(SUT.four + 12356, 12360)
		XCTAssertEqual(SUT.four + 12356, 12360)
		XCTAssertEqual(SUT.min + SUT.max, "-0.000000000000000001")
		XCTAssertEqual(SUT.pi + SUT.e, "5.859874482048838473")
		XCTAssertEqual(SUT.pi + SUT.e, SUT.e + SUT.pi) // commutative
	}

	func test_subtraction() {
		for item in SUT.positive {
			XCTAssertEqual(item - item, SUT.zero) // 3 - 3 => 0
		}
		for item in SUT.negative {
			XCTAssertEqual(item - item, SUT.zero) // (-3) - (-3) => (-3) + 3 => 0
		}

		for item in SUT.nonZero {
			// zero is identity under subtraction
			XCTAssertEqual(item - SUT.zero, item)
		}

		XCTAssertEqual(SUT.zero - 0, 0)
		XCTAssertEqual(SUT.zero - 1, -1)
		XCTAssertEqual(SUT.one - 1, 0)
		XCTAssertEqual(SUT.one - 2, -1)
		XCTAssertEqual(SUT.seven - 5, 2)
		XCTAssertEqual(SUT(12360) - 12356, SUT.four)
		XCTAssertEqual(SUT.pi - SUT.e, "0.423310825130748003")
		XCTAssertEqual(SUT.e - SUT.pi, "-0.423310825130748003")
		XCTAssertEqual(SUT.max - SUT.max, 0)
		XCTAssertEqual(SUT.min - SUT.min, 0)
	}

	func test_multiplication() {
		for item in SUT.nonZero {
			// `1` is identity under multiplication
			XCTAssertEqual(item * SUT.one, item)
		}

		for (lhs, rhs) in SUT.nonZero.slidingWindowPairs {
			XCTAssertEqual(lhs * rhs, rhs * lhs) // commutative
		}

		for item in SUT.nonZero {
			// Every number multiplied by zero, is zero...
			XCTAssertEqual(item * SUT.zero, SUT.zero)
		}
		// ... including `max` and `min`
		XCTAssertEqual(SUT.max * 0, 0)
		XCTAssertEqual(SUT.min * 0, 0)

		var sut: SUT = .ten
		sut *= SUT.five
		XCTAssertEqual(sut, 50)
	}

	func test_division() {
		XCTAssertEqual(SUT.nine / SUT.three, SUT.three)

		for item in SUT.nonZero {
			// All numbers divided by themselves equals `one`...
			XCTAssertEqual(item / item, SUT.one)
		}
		// ... including `max` and `min`
		XCTAssertEqual(SUT.max / SUT.max, SUT.one)
		XCTAssertEqual(SUT.min / SUT.min, SUT.one)
	}

	func test_is_negative() {
		for item in SUT.negative {
			XCTAssertTrue(item.isNegative)
		}
		for item in SUT.positive {
			XCTAssertFalse(item.isNegative)
		}
	}

	func test_is_positive() {
		for item in SUT.negative {
			XCTAssertFalse(item.isPositive)
		}
		for item in SUT.positive {
			XCTAssertTrue(item.isPositive)
		}
	}

	func test_is_zero() {
		for item in SUT.negative {
			XCTAssertFalse(item.isZero)
		}
		for item in SUT.positive {
			XCTAssertFalse(item.isZero)
		}

		XCTAssert(SUT.zero.isZero)
	}

	func test_clamped() {
		for item in SUT.negative {
			XCTAssertEqual(item.clamped, SUT.zero)
		}
		for item in SUT.positive {
			XCTAssertEqual(item.clamped, item)
		}
	}

	func test_exponent() {
		func doTest(exponent: UInt8, expected: SUT) {
			XCTAssertEqual(SUT(exponent: exponent), expected)
		}
		doTest(exponent: 0, expected: 1)
		doTest(exponent: 1, expected: 10)
		doTest(exponent: 2, expected: 100)
		doTest(exponent: 3, expected: 1000)
		doTest(exponent: 4, expected: 10000)
	}

	func test_negation() {
		XCTAssertEqual(-SUT.five, SUT.zero - 5)
	}

	func test_init_source_exactly() {
		XCTAssertEqual(SUT(exactly: UInt64(12_345_678_912_345_678)), 12_345_678_912_345_678)
		XCTAssertEqual(SUT(exactly: Int64(-12_345_678_912_345_678)), SUT("12345678912345678").negate())
	}

	func test_from_and_from_formatted() {
		func doTest(_ decimalString: String, line: UInt = #line) {
			XCTAssertNoThrow(
				try SUT(
					formattedString: decimalString,
					locale: .test
				),
				line: line
			)
			XCTAssertNoDifference(
				try SUT(
					formattedString: decimalString,
					locale: .test
				).formatted(locale: .test),
				decimalString,
				line: line
			)
		}
		doTest("123,456.98")
		doTest("0.1234")
		doTest("1,234.9876")
	}

	func test_rounded() {
		func doTest(_ from: SUT, decimalPlaces: UInt8, expected: SUT, line: UInt = #line) {
			let sut = from.rounded(decimalPlaces: decimalPlaces)
			XCTAssertEqual(sut, expected, line: line)
		}

		doTest(0.12345, decimalPlaces: 5, expected: 0.12345) // unchanged
		doTest(0.12345, decimalPlaces: 4, expected: 0.1235)
		doTest(0.12345, decimalPlaces: 3, expected: 0.123)
		doTest(0.12345, decimalPlaces: 2, expected: 0.12)
		doTest(0.12345, decimalPlaces: 1, expected: 0.1)
		doTest(0.12345, decimalPlaces: 0, expected: 0)
	}

	func test_ceil() {
		func doTest(_ from: SUT, decimalPlaces: UInt8, expected: SUT, line: UInt = #line) {
			let sut = from.ceil(decimalPlaces: decimalPlaces)
			XCTAssertEqual(sut, expected, line: line)
		}

		doTest(0.12345, decimalPlaces: 5, expected: 0.12345) // unchanged
		doTest(0.12345, decimalPlaces: 4, expected: 0.1235)
		doTest(0.12345, decimalPlaces: 3, expected: 0.124)
		doTest(0.12345, decimalPlaces: 2, expected: 0.13) // ceil: away from zero, so 0.13, not 0.12
		doTest(0.12345, decimalPlaces: 1, expected: 0.2)
		doTest(0.12345, decimalPlaces: 0, expected: 1)
	}

	func test_floor() {
		func doTest(_ from: SUT, decimalPlaces: UInt8, expected: SUT, line: UInt = #line) {
			let sut = from.floor(decimalPlaces: decimalPlaces)
			XCTAssertEqual(sut, expected, line: line)
		}

		doTest(0.12345, decimalPlaces: 5, expected: 0.12345) // unchanged
		doTest(0.12345, decimalPlaces: 4, expected: 0.1234)
		doTest(0.12345, decimalPlaces: 3, expected: 0.123)
		doTest(0.12345, decimalPlaces: 2, expected: 0.12)

		doTest(0.955, decimalPlaces: 3, expected: 0.955)
		doTest(0.955, decimalPlaces: 2, expected: 0.95)
		doTest(0.955, decimalPlaces: 1, expected: 0.9)
		doTest(0.955, decimalPlaces: 0, expected: 0)
	}

	func test_from_double() throws {
		func doTest(_ double: Double, _ expected: String) throws {
			let sut = try SUT(double)
			XCTAssertEqual(sut.toRawString(), expected)
		}
		try doTest(Double(Float32.greatestFiniteMagnitude), "340282346638528860000000000000000000000") // precision lost
		try doTest(0.1, "0.1")
		try doTest(4.012345678901234567895555555, "4.012345678901235")
	}

	func test_magnitude() {
		XCTAssertEqual(SUT.min.magnitude, SUT.max)
	}

	func test_standard_transaction_fee() {
		XCTAssertEqual(SUT.temporaryStandardFee, 25)
	}

	func test_decoding_to_SUT() throws {
		struct TestStruct: Codable, Equatable {
			let decimal: SUT
			let optional: SUT?
		}

		func doTest(_ string: String, decimal expectedDecimal: SUT, optionalIsNil: Bool = false) throws {
			if let data = string.data(using: .utf8) {
				let actual = try JSONDecoder().decode(TestStruct.self, from: data)
				let expected = TestStruct(decimal: expectedDecimal, optional: optionalIsNil ? nil : expectedDecimal)
				XCTAssertEqual(actual, expected)
			} else {
				XCTFail()
			}
		}

		try doTest("{\"decimal\":\"123.1234\",\"optional\":\"123.1234\"}", decimal: .init("123.1234"))
		try doTest("{\"decimal\":\"1233434.1234\",\"optional\":\"1233434.1234\"}", decimal: .init("1233434.1234"))
		try doTest("{\"decimal\":\"124300.1332\",\"optional\":\"124300.1332\"}", decimal: .init("124300.1332"))
		try doTest("{\"decimal\":\"000124300.1332000\",\"optional\":\"000124300.1332000\"}", decimal: .init("000124300.1332000"))
		try doTest("{\"decimal\":\"124300.000001332\",\"optional\":\"124300.000001332\"}", decimal: .init("124300.000001332"))
		try doTest("{\"decimal\":\"0.0000000223\",\"optional\":\"0.0000000223\"}", decimal: .init("0.0000000223"))
		try doTest("{\"decimal\":\"0.000\",\"optional\":\"0.000\"}", decimal: .init("0.000"))
		try doTest("{\"decimal\":\"0.0\",\"optional\":\"0.0\"}", decimal: .init("0.0"))
		try doTest("{\"decimal\":\"0.009999999999999\",\"optional\":\"0.009999999999999\"}", decimal: .init("0.009999999999999"))
		try doTest("{\"decimal\":\"1234123.4\",\"optional\":\"1234123.4\"}", decimal: .init("1234123.4"))
		try doTest("{\"decimal\":\"123456.34\",\"optional\":\"123456.34\"}", decimal: .init("123456.34"))
		try doTest("{\"decimal\":\"12345.234\",\"optional\":\"12345.234\"}", decimal: .init("12345.234"))

		try doTest("{\"decimal\":\"12341234\",\"optional\":\"12341234\"}", decimal: .init("12341234"))
		try doTest("{\"decimal\":\"1234123412341234\",\"optional\":\"1234123412341234\"}", decimal: .init("1234123412341234"))

		try doTest("{\"decimal\":\"00000123\",\"optional\":\"00000123\"}", decimal: .init("123"))
		try doTest("{\"decimal\":\"00000123.1234\",\"optional\":\"00000123.1234\"}", decimal: .init("123.1234"))
		try doTest("{\"decimal\":\"00000123.12340000\",\"optional\":\"00000123.12340000\"}", decimal: .init("123.1234"))
		try doTest("{\"decimal\":\"123.12340000\",\"optional\":\"123.12340000\"}", decimal: .init("123.1234"))

		try doTest("{\"decimal\":\"123.1234\"}", decimal: .init("123.1234"), optionalIsNil: true)
		try doTest("{\"decimal\":\"12341234\"}", decimal: .init("12341234"), optionalIsNil: true)
	}

	func test_roundtrip_coding_SUT() throws {
		struct TestStruct: Codable, Equatable {
			let decimal: SUT?
		}

		func doTest(_ decimal: SUT?) throws {
			let original = TestStruct(decimal: decimal)
			let encoded = try JSONEncoder().encode(original)
			let decoded = try JSONDecoder().decode(TestStruct.self, from: encoded)
			XCTAssertEqual(original, decoded)
		}

		try doTest(nil)

		for decimalString in smallDecimalStrings {
			let sut = try SUT(decimalString)
			try doTest(sut)
			let fromRawString = try SUT(sut.toRawString())
			XCTAssertNoDifference(sut, fromRawString)
		}
	}

	func test_as_double() throws {
		typealias LargeVector = (string: String, lostPrecision: UInt8)
		let largeDecimalsStrings: [LargeVector] = [
			(string: "000009876543212345678987654321.1415", lostPrecision: 10),
			(string: "123459876543212345678987654321.2370", lostPrecision: 15),
		]
		let numberFormatter = NumberFormatter()
		numberFormatter.maximumFractionDigits = 18
		numberFormatter.locale = .test

		func testSmall(_ string: String) throws {
			let sut = try SUT(string)
			let double = sut.asDouble
			let doubleFormatted = try XCTUnwrap(numberFormatter.string(for: double))
			XCTAssertEqual(sut.toRawString(), doubleFormatted)
		}

		func testLarge(_ vector: LargeVector) throws {
			let sut = try SUT(vector.string)
			let double = sut.asDouble
			let doubleFormatted = try XCTUnwrap(numberFormatter.string(for: double))
			let scale = SUT(exponent: vector.lostPrecision)
			let rounded = (sut / scale).rounded(decimalPlaces: 0) * scale
			XCTAssertEqual(rounded.toRawString(), doubleFormatted)
		}

		try smallDecimalStrings.forEach(testSmall)
		try largeDecimalsStrings.forEach(testLarge)

		XCTAssertLessThan(SUT.min.asDouble, SUT.max.asDouble)
		XCTAssertNoThrow(SUT("12345678987654321.000000000000000001").asDouble)
	}

	private var smallDecimalStrings: [String] {
		[
			"0.000000000000000001",
			"123.1234",
			"1233434.1234",
			"124300.1332",
			"000124300.1332000",
			"124300.000001332",
			"0.0000000223",
			"0.000",
			"0.0",
			"0.009999999999999",
			"12341234",
			"1234123.4",
			"123456.34",
			"12345.234",
			"0.00000009",
			"0.000000009",
			"12.3456789",
			"0.123456789",
			"0.4321",
			"0.0000000000001",
			"0.9999999999999",
			"1000",
			"1000.01",
			"1000.123456789",
			"1000000.1234",
			"10000000.1234",
			"10000000.5234",
			"999.999999999943",
			"-0.123456789",
			"-0.4321",
			"-0.0000000000001",
			"-0.9999999999999",
			"-1000",
			"-1000.01",
			"-1000.123456789",
			"-1000000.1234",
			"1",
			"0.0",
			"1.0",
		]
	}

	func test_from_double_zeroPrice() throws {
		try doTestFromDouble(0, expected: SUT.zero)
	}

	func test_from_double_noDecimalPlaces_1() throws {
		try doTestFromDouble(10, expected: 10)
	}

	func test_from_double_noDecimalPlaces_2() throws {
		try doTestFromDouble(10000, expected: 10000)
	}

	func test_from_double_noDecimalPlaces_3() throws {
		try doTestFromDouble(10_000_000, expected: 10_000_000)
	}

	func test_from_double_withDecimalPlaces_1() throws {
		try doTestFromDouble(1.99, expected: SUT("1.99"))
	}

	func test_from_double_withDecimalPlaces_2() throws {
		try doTestFromDouble(1.000099, expected: SUT("1.000099"))
	}

	func test_from_double_belowOne_1() throws {
		try doTestFromDouble(0.99, expected: 0.99)
	}

	func test_from_double_belowOne_2() throws {
		try doTestFromDouble(0.000099, expected: SUT("0.000099"))
	}

	func test_from_double_closeToSUTDivisibility() throws {
		// 17 decimal places
		try doTestFromDouble(
			1.12345678901234567,
			expected: SUT("1.1234567890123457")
		)
	}

	func test_from_double_maxSUTDivisibility() throws {
		// 18 decimal places
		try doTestFromDouble(1.123456789012345678, expected: SUT("1.1234567890123457"))
	}

	func test_from_double_overMaxSUTDivisibility() throws {
		// 22 decimal places
		try doTestFromDouble(1.1234567890123456789012, expected: SUT("1.1234567890123457"))
	}

	func test_from_double_large_value() throws {
		try doTestFromDouble(
			70_000_000_000.987654,
			expected: SUT("70000000000.98766")
		)
	}

	private func doTestFromDouble(
		_ double: Double,
		expected: SUT,
		file: StaticString = #filePath,
		line: UInt = #line
	) throws {
		let fromDouble = try SUT(double)
		XCTAssertEqual(
			fromDouble,
			expected,
			"expected \(expected.formattedPlain(locale: .test)), got \(fromDouble.formattedPlain(locale: .test))",
			file: file,
			line: line
		)
	}
}
