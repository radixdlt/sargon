@_exported import Foundation
@_exported import Sargon
@_exported import XCTest

class Test<SUT_: SargonModel>: XCTestCase {
	typealias SUT = SUT_
	
	func test_equality() throws {
		XCTAssertEqual(SUT.sample, SUT.sample)
	}

	func test_inequality() throws {
		XCTAssertNotEqual(SUT.sample, SUT.sampleOther)
	}

	func test_custom_string_convertible() throws {
		XCTAssertEqual(SUT.sample.description, SUT.sample.description)
		XCTAssertEqual(SUT.sampleOther.description, SUT.sampleOther.description)
	}
	
}
