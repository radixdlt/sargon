
final class Secp256k1PublicKeyTests: PublicKeyTest<Secp256k1PublicKey> {
	func test_from_compressed() throws {
		// from K1: https://github.com/Sajjon/K1/blob/main/Tests/K1Tests/TestCases/Keys/PublicKey/PublicKeyImportTests.swift#L48
		XCTAssertNoThrow(try SUT(hex: "020202020202020202020202020202020202020202020202020202020202020202"))
	}
	
	func test_from_uncompressed() throws {
		// from K1: https://github.com/Sajjon/K1/blob/main/Tests/K1Tests/TestCases/Keys/PublicKey/PublicKeyImportTests.swift#L48
		XCTAssertNoThrow(try SUT(hex: "040202020202020202020202020202020202020202020202020202020202020202415456f0fc01d66476251cab4525d9db70bfec652b2d8130608675674cde64b2"))
	}
}
