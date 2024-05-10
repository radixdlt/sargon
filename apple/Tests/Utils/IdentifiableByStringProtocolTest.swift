import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class IdentifiableByStringProtocolTest<SUT_: IdentifiableByStringProtocol>: Test<SUT_> {
    
    func test_string_roundtrip_symmetric_with_raw() throws {
        try eachSample { sut in
            let roundtripped = try SUT(sut.toRawString())
            XCTAssertEqual(sut, roundtripped)
        }
    }
    
	/// Cyon: We might be able remove this function once we have converted to `swift-testing` which has much more 
	/// powerful discovery than XCTest, and maybe `eachSampleCodableRoundtripTest` will be picked up as
	/// a test directly.
	func testJSONRoundtripAllSamples() throws {
		try eachSampleCodableRoundtripTest()
	}
    
    func test_formatted_raw_is_raw() {
        eachSample { sut in
            XCTAssertEqual(sut.toRawString(), sut.formatted(.raw))
        }
    }
}

