final class NotarySignatureTests: Test<NotarySignature> {
	func test_signature_roundtrip() {
		XCTAssertEqual(SUT(signature: SUT.sample.signature), SUT.sample)
		XCTAssertEqual(SUT(signature: SUT.sampleOther.signature), SUT.sampleOther)
	}
}
