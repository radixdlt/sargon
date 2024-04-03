class TransactionHashProtocolTest<SUT_: TransactionHashProtocol>: IdentifiableByStringProtocolTest<SUT_> {
    
    func test_bech32EncodedTxId_is_raw() {
        func doTest(_ sut: SUT) {
            XCTAssertEqual(sut.bech32EncodedTxId, sut.toRawString())
        }
        SUT.allCases.forEach(doTest)
    }
}
