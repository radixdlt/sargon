extension Decimal192 {
	public static let pi: Self 	= "3.141592653589793238"
	public static let e: Self 		= "2.718281828459045235"
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
			Self(21_000_000)
		]
		assert(values.sorted() == values)
		assert(Set(values).count == values.count) // assert no duplicates
		return values
	}()
	
	// Sorted in increasing order: [-10, -9, .. -2, -1]
	public static let negative: [Self] = {
		positive.map { $0.negate() }.sorted()
	}()
	
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
		enumerated().compactMap { (offset, element) in
			let nextIndex = offset + 1
			if nextIndex >= count {
				return nil
			}
			let nextElement = self[nextIndex]
			return (element, nextElement)
		}
	}
}

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
		SUT.positive.identityPairs.forEach { lhs, rhs in
			XCTAssertEqual(lhs, rhs)
		}
		SUT.negative.identityPairs.forEach { lhs, rhs in
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
		SUT.nonZero.identityPairs.forEach { lhs, rhs in
			XCTAssertGreaterThanOrEqual(lhs, rhs)
		}
		SUT.nonZero.slidingWindowPairs.forEach { lhs, rhs in
			XCTAssertGreaterThanOrEqual(rhs, lhs)
		}
		XCTAssertGreaterThanOrEqual(SUT.four, SUT.three)
		XCTAssertGreaterThanOrEqual(SUT.five, SUT.five)
		XCTAssertGreaterThanOrEqual(SUT.pi, SUT.e)
	}
	
	func test_less_than() {
		SUT.nonZero.slidingWindowPairs.forEach { lhs, rhs in
			XCTAssertLessThan(lhs, rhs)
		}
		XCTAssertLessThan(SUT.nine, SUT.ten)
		XCTAssertLessThan(SUT.min, SUT.zero)
		XCTAssertLessThan(SUT.zero, SUT.max)
		XCTAssertLessThan(SUT.e, SUT.pi)
	}
	
	func test_less_than_or_equal() {
		SUT.nonZero.identityPairs.forEach { lhs, rhs in
			XCTAssertLessThanOrEqual(lhs, rhs)
		}
		XCTAssertLessThanOrEqual(SUT.seven, SUT.eight)
		XCTAssertLessThanOrEqual(SUT.six, SUT.six)
		XCTAssertLessThanOrEqual(SUT.e, SUT.pi)
	}
	
	func test_addition() {
		SUT.nonZero.slidingWindowPairs.forEach { lhs, rhs in
			XCTAssertEqual(lhs + rhs, rhs + lhs) // commutative
		}
		SUT.negative.forEach {
			XCTAssertEqual($0 + $0, 2 * $0)
		}
		
		SUT.nonZero.forEach {
			// zero is identity under addition
			XCTAssertEqual($0 + SUT.zero, $0)
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
		SUT.positive.forEach {
			XCTAssertEqual($0 - $0, SUT.zero) // 3 - 3 => 0
		}
		SUT.negative.forEach {
			XCTAssertEqual($0 - $0, SUT.zero) // (-3) - (-3) => (-3) + 3 => 0
		}
		
		SUT.nonZero.forEach {
			// zero is identity under subtraction
			XCTAssertEqual($0 - SUT.zero, $0)
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
		
		SUT.nonZero.forEach {
			// `1` is identity under multiplication
			XCTAssertEqual($0 * SUT.one, $0)
		}
		
		SUT.nonZero.slidingWindowPairs.forEach { lhs, rhs in
			XCTAssertEqual(lhs * rhs, rhs * lhs) // commutative
		}
		
		SUT.nonZero.forEach {
			// Every number multiplied by zero, is zero...
			XCTAssertEqual($0 * SUT.zero, SUT.zero)
		}
		// ... incliding `max` and `min`
		XCTAssertEqual(SUT.max * 0, 0)
		XCTAssertEqual(SUT.min * 0, 0)
		
		var sut: SUT = .ten
		sut *= SUT.five
		XCTAssertEqual(sut, 50)
	}
	
	func test_division() {
		XCTAssertEqual(SUT.nine / SUT.three, SUT.three)
		
		SUT.nonZero.forEach {
			// All numbers divided by themselves equals `one`...
			XCTAssertEqual($0 / $0, SUT.one)
		}
		// ... incliding `max` and `min`
		XCTAssertEqual(SUT.max / SUT.max, SUT.one)
		XCTAssertEqual(SUT.min / SUT.min, SUT.one)
	}
	
	func test_is_negative() {
		SUT.negative.forEach {
			XCTAssertTrue($0.isNegative())
		}
		SUT.positive.forEach {
			XCTAssertFalse($0.isNegative())
		}
	}
	
	func test_clamped() {
		SUT.negative.forEach {
			XCTAssertEqual($0.clamped, SUT.zero)
		}
		SUT.positive.forEach {
			XCTAssertEqual($0.clamped, $0)
		}
	}
	
	func test_negation() {
		XCTAssertEqual(-SUT.five, SUT.zero - 5)
	}
	
	func test_init_source_exactly() {
		XCTAssertEqual(SUT(exactly: UInt64(12345678912345678)), 12345678912345678)
		XCTAssertEqual(SUT(exactly: Int64(-12345678912345678)), SUT("12345678912345678").negate())
	}
	
	func test_from_and_from_formatted() {
		func doTest(_ decimalString: String , line: UInt = #line) {
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
	
	func test_magnitude() {
		XCTAssertEqual(SUT.min.magnitude, SUT.max)
	}
}
