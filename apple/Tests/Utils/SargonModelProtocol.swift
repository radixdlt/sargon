@_exported import Foundation
@_exported import Sargon
@_exported import XCTest
@_exported import CustomDump

class TestCase: XCTestCase {
	override func setUp() {
		self.continueAfterFailure = false
	}
}

class Test<SUT_: SargonModel>: TestCase {
	typealias SUT = SUT_
	
	func test_equality() throws {
		XCTAssertNoDifference(SUT.sample, SUT.sample)
	}

	func test_inequality() throws {
		XCTAssertNotEqual(SUT.sample, SUT.sampleOther)
	}
	
	func test_hashable() {
		XCTAssertNoDifference(Set([SUT.sample, SUT.sample]).count, 1)
		XCTAssertNoDifference(Set([SUT.sampleOther, SUT.sampleOther]).count, 1)
		
		var set = Set<SUT>()
		SUT.allCases.forEach { set.insert($0) }
		SUT.allCases.forEach { set.insert($0) } // duplicates removed.
		XCTAssertGreaterThanOrEqual(set.count, 2)
	}

	func test_custom_string_convertible() throws {
		guard
			let sample = SUT.sample as? CustomStringConvertible,
			let sampleOther = SUT.sample as? CustomStringConvertible
		else {
			return
		}
		XCTAssertNoDifference(sample.description, sample.description)
		XCTAssertNoDifference(sampleOther.description, sampleOther.description)
	}
	
}
