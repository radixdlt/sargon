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
}
