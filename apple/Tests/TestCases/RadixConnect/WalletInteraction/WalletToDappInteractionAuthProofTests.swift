import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class WalletToDappInteractionAuthProofTests: TestCase {
	typealias SUT = WalletToDappInteractionAuthProof

	func testNewFromSignatureWithPublicKey_Ed25519() {
		let signatureWithPublicKey = SignatureWithPublicKey.sample // Ed25519
		let sut = SUT(signatureWithPublicKey: signatureWithPublicKey)
		XCTAssertEqual(sut.curve, .curve25519)
		XCTAssertEqual(sut.publicKey, signatureWithPublicKey.publicKey)
		XCTAssertEqual(sut.signature, signatureWithPublicKey.signature)
	}

	func testNewFromSignatureWithPublicKey_Secp256k1() {
		let signatureWithPublicKey = SignatureWithPublicKey.sampleOther // Secp256k1
		let sut = SUT(signatureWithPublicKey: signatureWithPublicKey)
		XCTAssertEqual(sut.curve, .secp256k1)
		XCTAssertEqual(sut.publicKey, signatureWithPublicKey.publicKey)
		XCTAssertEqual(sut.signature, signatureWithPublicKey.signature)
	}

	func testNewFromIntentSignatureOfOwner_Ed25519() {
		let intentSignature = IntentSignature.sample // Ed25519
		let sut = SUT(intentSignatureOfOwner: .init(owner: .sample, intentSignature: intentSignature))
		XCTAssertEqual(sut.curve, .curve25519)
		XCTAssertEqual(sut.publicKey, intentSignature.signatureWithPublicKey.publicKey)
		XCTAssertEqual(sut.signature, intentSignature.signatureWithPublicKey.signature)
	}

	func testNewFromIntentSignatureOfOwner_Secp256k1() {
		let intentSignature = IntentSignature.sampleOther // Secp256k1
		let sut = SUT(intentSignatureOfOwner: .init(owner: .sample, intentSignature: intentSignature))
		XCTAssertEqual(sut.curve, .secp256k1)
		XCTAssertEqual(sut.publicKey, intentSignature.signatureWithPublicKey.publicKey)
		XCTAssertEqual(sut.signature, intentSignature.signatureWithPublicKey.signature)
	}
}
