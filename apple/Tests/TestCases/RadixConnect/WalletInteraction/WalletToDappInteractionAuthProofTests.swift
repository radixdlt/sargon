import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class WalletToDappInteractionAuthProofTests: TestCase {
	typealias SUT = WalletToDappInteractionAuthProof

	func testNewFromIntentSignatures_Ed25519() throws {
		let intentSignature = IntentSignature.sample // Ed25519
		let sut = SUT(intentSignatureOfOwner: .init(owner: .sample, intentSignature: intentSignature))
		XCTAssertEqual(sut.curve, .curve25519)
		XCTAssertEqual(sut.publicKey, intentSignature.signatureWithPublicKey.publicKey)
		XCTAssertEqual(sut.signature, intentSignature.signatureWithPublicKey.signature)
	}

	func testNewFromIntentSignatures_Secp256k1() throws {
		let intentSignature = IntentSignature.sampleOther // Secp256k1
		let sut = SUT(intentSignatureOfOwner: .init(owner: .sample, intentSignature: intentSignature))
		XCTAssertEqual(sut.curve, .secp256k1)
		XCTAssertEqual(sut.publicKey, intentSignature.signatureWithPublicKey.publicKey)
		XCTAssertEqual(sut.signature, intentSignature.signatureWithPublicKey.signature)
	}
}
