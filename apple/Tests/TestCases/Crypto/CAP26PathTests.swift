final class CAP26PathTests: HDPathProtocolTests<CAP26Path> {
    func test_sample_description() {
        XCTAssertNoDifference(SUT.sample.description, "m/44H/1022H/1H/525H/1460H/0H")
    }
    
    func test_sample_other_description() {
        XCTAssertNoDifference(SUT.sampleOther.description, "m/44H/1022H/1H/618H/1460H/0H")
    }
    
    func test_sample_from_str() {
        XCTAssertNoDifference(
            "m/44H/1022H/1H/525H/1460H/0H",
            SUT.sample
        )
    }
	
	func test_invalid_got_bip44_like_legacy_path() {
		XCTAssertThrowsError(try SUT(string: "m/44H/1022H/0H/0/0H"))
	}
}
