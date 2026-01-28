import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

final class TransactionManifestV2Tests: Test<TransactionManifestV2> {
	func test_manifest_string() {
		let manifest = SUT.sample
		XCTAssert(manifest.manifestString.contains("CALL_METHOD"))
	}

	func test_manifest_network_id() {
		let manifest = SUT.sample
		XCTAssertNoDifference(manifest.networkId, .mainnet)
	}

	func test_manifest_blobs() {
		let manifest = SUT.sample
		XCTAssertNoDifference(manifest.blobs, [])
	}

	func test_involved_resource_addresses() {
		XCTAssertNoDifference(SUT.sample.involvedResourceAddresses, [ResourceAddress.sampleMainnetXRD])
	}

	func test_involved_pool_addresses() {
		XCTAssertNoDifference(SUT.sample.involvedPoolAddresses, [])
	}

	func test_manifest_summary() {
		XCTAssertNoDifference(SUT.sample.summary?.addressesOfAccountsWithdrawnFrom, [AccountAddress.sampleMainnet])
	}
}
