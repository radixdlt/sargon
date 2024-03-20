@_exported import Foundation
@_exported import Sargon
@_exported import XCTest
@_exported import CustomDump

class Test<SUT_: SargonModel>: XCTestCase {
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
		XCTAssertNoDifference(Set([SUT.sample, SUT.sampleOther, SUT.sampleOther, SUT.sample]).count, 2)
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
