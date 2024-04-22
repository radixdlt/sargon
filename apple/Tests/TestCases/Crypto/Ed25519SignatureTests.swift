import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class Ed25519SignatureTests: SignatureTest<Ed25519Signature> {
	func test_from_exactly_64_bytes() {
		XCTAssertEqual(SUT(exactly: SUT.sample.bytes), SUT.sample)
	}
	
	func test_as_signature() {
		let sut = SUT.sample
		XCTAssertEqual(sut.signature, Signature.ed25519(value: sut))
	}
}

