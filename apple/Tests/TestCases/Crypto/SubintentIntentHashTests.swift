import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SubintentHashTests: TestCase {
	typealias SUT = SubintentHash
	
	func test_network_id() {
		XCTAssertEqual(SUT.sample.networkId, .mainnet)
	}
	
	func test_network_id_other() {
		XCTAssertEqual(SUT.sampleOther.networkId, .simulator)
	}
	
	func test_formatted_default() {
        XCTAssertNoDifference(SUT.sample.formatted(), "subt...y6hgte")
    }
}
