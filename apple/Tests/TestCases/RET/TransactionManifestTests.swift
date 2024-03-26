@testable import Sargon

final class TransactionManifestTests: Test<TransactionManifest> {

    func test_manifest_instructions_string() {
		let manifest = TransactionManifest.sample
		XCTAssert(manifest.instructionsString.contains("CALL_METHOD"))
	}

    func test_manifest_network_id() {
		let manifest = TransactionManifest.sample
        XCTAssertNoDifference(manifest.networkID, .mainnet)
	}

    func test_manifest_blobs() {
		let manifest = TransactionManifest.sample
        XCTAssertNoDifference(manifest.blobs, [])
	}
    
    func test_involved_resource_addresses() {
        XCTAssertNoDifference(SUT.sample.involvedResourceAddresses, [ResourceAddress.sampleMainnetXRD])
    }
    
    func test_involved_pool_addresses() {
        XCTAssertNoDifference(SUT.sample.involvedPoolAddresses, [])
    }
    
    func test_manifest_summary() {
        XCTAssertNoDifference(SUT.sample.summary.addressesOfAccountsWithdrawnFrom, [AccountAddress.sampleMainnet])
    }
    
    func test_execution_summary() throws {
        let name = "transfer_1to2_multiple_nf_and_f_tokens"
        let receipt = try encodedReceipt(name)
        let manifest = try rtm(name)
        
        let summary = try manifest.executionSummary(encodedReceipt: receipt)
        
        XCTAssertNoDifference(summary.addressesOfAccountsRequiringAuth, ["account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk"])
    }
}
