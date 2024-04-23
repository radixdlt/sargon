import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class NetworkDefinitionTests: Test<NetworkDefinition> {
	func test_lookup_mainnet() throws {
		try XCTAssertEqual(
			NetworkDefinition.lookupBy(logicalName: "mainnet").id,
			.mainnet
		)
	}
	
	func test_lookup_stokenet() throws {
		try XCTAssertEqual(
			NetworkDefinition.lookupBy(logicalName: "stokenet").id,
			.stokenet
		)
	}
}
