import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AuthorizedPersonaSimpleTests: Test<AuthorizedPersonaSimple> {
	func test_network_ids_mainnet() {
		XCTAssertTrue(SUT.sampleValuesMainnet.allSatisfy { $0.networkID == .mainnet })
	}

	func test_network_ids_stokenet() {
		XCTAssertTrue(SUT.sampleValuesStokenet.allSatisfy { $0.networkID == .stokenet })
	}
}
