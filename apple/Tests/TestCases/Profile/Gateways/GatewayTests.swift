import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class GatewayTests: Test<Gateway> {
	func test_is_wellknown() {
		XCTAssertTrue(SUT.mainnet.isWellknown)
		XCTAssertTrue(SUT.stokenet.isWellknown)
		XCTAssertFalse(SUT.forNetwork(id: .hammunet).isWellknown)
	}
	
	func test_new_url_network_id() throws {
		XCTAssertEqual(
			SUT.mainnet,
			try SUT(
				url: "https://mainnet.radixdlt.com",
				networkID: .mainnet
			)
		)
	}
	
	func test_id() throws {
		XCTAssertEqual(SUT.sample.id, SUT.sample.getID())
	}
	
	func test_description() throws {
		XCTAssertEqual(SUT.sample.description, SUT.sample.toString())
	}

}
