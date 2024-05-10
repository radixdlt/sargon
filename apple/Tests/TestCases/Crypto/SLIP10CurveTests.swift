import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SLIP10CurveTests: Test<SLIP10Curve> {
	func test_description() {
		XCTAssertNoDifference(SUT.sample.toString(), SUT.sample.description)
		XCTAssertNoDifference(SUT.sampleOther.toString(), SUT.sampleOther.description)
	}
	
	func test_string_roundtrip() throws {
		try eachSample { sut in
			let string = sut.toString()
			let fromString = try XCTUnwrap(SUT.init(rawValue: string))
			XCTAssertEqual(fromString, sut)
		}
	}
	
	/// Cyon: We might be able remove this function once we have converted to `swift-testing` which has much more 
	/// powerful discovery than XCTest, and maybe `eachSampleCodableRoundtripTest` will be picked up as
	/// a test directly.
	func testJSONRoundtripAllSamples() throws {
		try eachSampleCodableRoundtripTest()
	}
}
