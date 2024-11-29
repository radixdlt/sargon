import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class SpecificFactorSourceIDTest<SUT_: FactorSourceIDSpecificProtocol>: FactorSourceIDTest<SUT_> {
	func test_extract() throws {
		try eachSample { sut in
			let embedded = sut.asGeneral
			let extracted: SUT = try embedded.extract()
			XCTAssertEqual(extracted, sut)
		}
	}

	/// Cyon: We might be able remove this function once we have converted to `swift-testing` which has much more
	/// powerful discovery than XCTest, and maybe `eachSampleCodableRoundtripTest` will be picked up as
	/// a test directly.
	func testJSONRoundtripAllSamples() throws {
		try eachSampleCodableRoundtripTest()
	}
}
