final class IntentSignatureTests: Test<IntentSignature> {
    func test_get_signature_with_public_key() {
        let signature = SignatureWithPublicKey.sample
        XCTAssertEqual(
            SUT(signatureWithPublicKey: signature).signatureWithPublicKey,
            signature
        )
    }
}
