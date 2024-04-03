final class IntentHashTests: TransactionHashProtocolTest<IntentHash> {
	func test_network_id() {
		XCTAssertEqual(SUT.sample.networkID, .mainnet)
	}
	
	func test_network_id_other() {
		XCTAssertEqual(SUT.sampleOther.networkID, .simulator)
	}
	
	func test_string_roundtrip() {
		let s = "txid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6szm3ltd"
        XCTAssertEqual(try SUT.init(s).description, s)
	}
    
    func test_formatted_default() {
        XCTAssertNoDifference(SUT.sample.formatted(), "txid...zm3ltd")
    }
}
