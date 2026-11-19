import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class GatewayTests: Test<Gateway> {
	func test_is_wellknown() throws {
		XCTAssertTrue(SUT.mainnet.isWellknown)
		XCTAssertTrue(SUT.stokenet.isWellknown)
		XCTAssertFalse(try SUT(url: "https://hammunet-gateway.radixdlt.com", networkID: .hammunet).isWellknown)
		XCTAssertFalse(try SUT(url: "https://example.com", networkID: .mainnet).isWellknown)
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

	func test_id() {
		XCTAssertEqual(SUT.sample.id, SUT.sample.getID())
	}

	func test_description() {
		XCTAssertEqual(SUT.sample.description, SUT.sample.toString())
	}
}
