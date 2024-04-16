import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AuthorizedDappTests: Test<AuthorizedDapp> {
	
	func test_network_ids_mainnet() {
		XCTAssertTrue(SUT.sampleValuesMainnet.allSatisfy({ $0.networkID == .mainnet }))
	}
	
	func test_network_ids_stokenet() {
		XCTAssertTrue(SUT.sampleValuesStokenet.allSatisfy({ $0.networkID == .stokenet }))
	}
	
	func test_id_is_dapp_definition() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.id, sut.dAppDefinitionAddress)
		}
		SUT.sampleValues.forEach(doTest)
	}
}
