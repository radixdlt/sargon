class IdentifiableByStringProtocolTest<SUT_: IdentifiableByStringProtocol>: Test<SUT_> {
    
    func test_string_roundtrip_symmetric_with_raw() throws {
        func doTest(_ sut: SUT) throws {
            let roundtripped = try SUT(sut.toRawString())
            XCTAssertEqual(sut, roundtripped)
        }
        try SUT.allCases.forEach(doTest)
    }
    
    func test_codable_roundtrip() throws {
        try SUT.allCases.forEach(doTestCodableRoundtrip)
    }
    
    func test_formatted_raw_is_raw() {
        func doTest(_ sut: SUT) {
            XCTAssertEqual(sut.toRawString(), sut.formatted(.raw))
        }
        SUT.allCases.forEach(doTest)
    }
}

