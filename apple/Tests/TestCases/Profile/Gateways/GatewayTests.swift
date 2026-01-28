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

	func test_id() {
		XCTAssertEqual(SUT.sample.id, SUT.sample.getID())
	}

	func test_description() {
		XCTAssertEqual(SUT.sample.description, SUT.sample.toString())
	}

	func test_network_id_of_nebunet() {
		XCTAssertEqual(SUT.nebunet.networkID, .nebunet)
	}

	func test_network_id_of_kisharnet() {
		XCTAssertEqual(SUT.kisharnet.networkID, .kisharnet)
	}

	func test_network_id_of_ansharnet() {
		XCTAssertEqual(SUT.ansharnet.networkID, .ansharnet)
	}

	func test_network_id_of_hammunet() {
		XCTAssertEqual(SUT.hammunet.networkID, .hammunet)
	}

	func test_network_id_of_enkinet() {
		XCTAssertEqual(SUT.enkinet.networkID, .enkinet)
	}

	func test_network_id_of_mardunet() {
		XCTAssertEqual(SUT.mardunet.networkID, .mardunet)
	}
}
