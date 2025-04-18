import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class HDPathProtocolTest<SUT_: HDPathProtocol & SargonModel>: Test<SUT_> {
	func test_string_roundtrip() throws {
		try SUT.sampleValues.forEach {
			XCTAssertNoDifference(try SUT(string: $0.description), $0)
		}
	}
}
