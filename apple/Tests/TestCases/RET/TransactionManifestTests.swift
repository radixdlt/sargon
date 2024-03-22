final class TransactionManifestTests: Test<TransactionManifest> {

    func test_manifest_instructions_string() {
		
		let manifest = TransactionManifest.sample
		
		XCTAssert(manifest.instructionsString.contains("CALL_METHOD"))
	}

    func test_manifest_network_id() {
		
		let manifest = TransactionManifest.sample
		
		XCTAssertEqual(manifest.networkID, .mainnet)
	}

    func test_manifest_blobs() {
		
		let manifest = TransactionManifest.sample
		
		XCTAssertEqual(manifest.blobs, newBlobsFromBlobList(blobs: []))
	}
}
