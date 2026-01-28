import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class WalletToDappInteractionProofOfOwnershipTests: TestCase {
	typealias SUT = WalletToDappInteractionProofOfOwnership

	func testNewFromIntentSignatures_Ed25519_Account() {
		let owner = AddressOfAccountOrPersona.account(.sample)
		let intentSignature = IntentSignature.sample // Ed25519
		let sut = SUT(intentSignatureOfOwner: .init(owner: owner, intentSignature: intentSignature))
		switch sut {
		case let .account(value):
			XCTAssertEqual(value.proof.curve, .curve25519)
			XCTAssertEqual(value.proof.publicKey, intentSignature.signatureWithPublicKey.publicKey)
			XCTAssertEqual(value.proof.signature, intentSignature.signatureWithPublicKey.signature)
		case .persona:
			XCTFail("Expected account proof")
		}
	}

	func testNewFromIntentSignatures_Ed25519_Persona() {
		let owner = AddressOfAccountOrPersona.identity(.sample)
		let intentSignature = IntentSignature.sample // Ed25519
		let sut = SUT(intentSignatureOfOwner: .init(owner: owner, intentSignature: intentSignature))
		switch sut {
		case .account:
			XCTFail("Expected persona proof")
		case let .persona(value):
			XCTAssertEqual(value.proof.curve, .curve25519)
			XCTAssertEqual(value.proof.publicKey, intentSignature.signatureWithPublicKey.publicKey)
			XCTAssertEqual(value.proof.signature, intentSignature.signatureWithPublicKey.signature)
		}
	}

	func testNewFromIntentSignatures_Secp256k1_Account() {
		let owner = AddressOfAccountOrPersona.account(.sample)
		let intentSignature = IntentSignature.sampleOther // Secp256k1
		let sut = SUT(intentSignatureOfOwner: .init(owner: owner, intentSignature: intentSignature))
		switch sut {
		case let .account(value):
			XCTAssertEqual(value.proof.curve, .secp256k1)
			XCTAssertEqual(value.proof.publicKey, intentSignature.signatureWithPublicKey.publicKey)
			XCTAssertEqual(value.proof.signature, intentSignature.signatureWithPublicKey.signature)
		case .persona:
			XCTFail("Expected account proof")
		}
	}

	func testNewFromIntentSignatures_Secp256k1_Persona() {
		let owner = AddressOfAccountOrPersona.identity(.sample)
		let intentSignature = IntentSignature.sampleOther // Secp256k1
		let sut = SUT(intentSignatureOfOwner: .init(owner: owner, intentSignature: intentSignature))
		switch sut {
		case .account:
			XCTFail("Expected persona proof")
		case let .persona(value):
			XCTAssertEqual(value.proof.curve, .secp256k1)
			XCTAssertEqual(value.proof.publicKey, intentSignature.signatureWithPublicKey.publicKey)
			XCTAssertEqual(value.proof.signature, intentSignature.signatureWithPublicKey.signature)
		}
	}
}
