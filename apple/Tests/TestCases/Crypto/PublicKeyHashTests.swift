import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

final class PublicKeyHashTests: Test<PublicKeyHash> {
	
	func test_hash_public_key_ed25519() {
		XCTAssertNoDifference(SUT.hash(publicKey: .ed25519(.sample)), SUT.sample)
		XCTAssertNoDifference(Ed25519PublicKey.sample.hash(), SUT.sample)
	}
	
	func test_hash_public_key_secp256k1() {
		XCTAssertNoDifference(SUT.hash(publicKey: .secp256k1(.sample)), SUT.sampleOther)
		XCTAssertNoDifference(Secp256k1PublicKey.sample.hash(), SUT.sampleOther)
	}
	
	func test_hashing_init() {
		XCTAssertNoDifference(SUT(hashing: .secp256k1(.sample)), SUT.sampleOther)
	}
	
	func test_data() {
		XCTAssertEqual(SUT.sample.data.hex, "f4e18c034e069baee91ada4764fdfcf2438b8f976861df00557d4cc9e7")
		XCTAssertEqual(SUT.sampleOther.data.hex, "4a5004504dbbc08c65ba86fcd7592a3ac48db81d217fe2356e75b37f31")
	}
	
}
