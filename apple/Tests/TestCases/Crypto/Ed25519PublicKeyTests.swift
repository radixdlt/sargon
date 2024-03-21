@testable import Sargon
import CryptoKit

final class Ed25519PublicKeyTests: PublicKeyTest<Ed25519PublicKey> {
	func test_cryptokit_interop() throws {
		XCTAssertNoThrow(try SUT(hex: Curve25519.Signing.PrivateKey().publicKey.rawRepresentation.hex))
	}
}
