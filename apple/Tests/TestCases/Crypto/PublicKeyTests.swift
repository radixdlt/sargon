@testable import Sargon
import CryptoKit

final class PublicKeyTests: PublicKeyTest<PublicKey> {
	func test_cryptokit_interop() throws {
		XCTAssertNoThrow(try SUT(hex: Curve25519.Signing.PrivateKey().publicKey.rawRepresentation.hex))
		XCTAssertNoThrow(try Curve25519.Signing.PublicKey(rawRepresentation: SUT.sample.data))
	}
	
	func test_expressible_by_string_literal() {
		// secp256k1
		// expected public key uncompressed:
		// `043083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8ab3efd3320b8f893cb421ed7ff0aa9ff43b43cad4e00e194f89845c6ac8233a7`
		// expected public key compressed:
		// `033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8`
		XCTAssertEqual(SUT.sampleOther, "043083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8ab3efd3320b8f893cb421ed7ff0aa9ff43b43cad4e00e194f89845c6ac8233a7")
		
	}
	
	func test_embed_is_identity() {
		XCTAssertEqual(SUT.sample, SUT.sample.embed())
	}
}
