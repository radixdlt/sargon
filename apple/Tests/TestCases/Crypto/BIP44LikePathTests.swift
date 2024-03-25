final class BIP44LikePathTests: HDPathProtocolTests<BIP44LikePath> {
    func test_sample_description() {
        XCTAssertNoDifference(SUT.sample.description, "m/44H/1022H/0H/0/0H")
    }
    
    func test_sample_from_str() {
        XCTAssertNoDifference(
            "m/44H/1022H/0H/0/0H", // ExpressibleByStringLiteral
            SUT.sample
        )
    }
}
