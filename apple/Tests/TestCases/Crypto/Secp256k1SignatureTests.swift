import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class Secp256k1SignatureTests: SignatureTest<Secp256k1Signature> {
	
	func test_from_exactly_65_bytes() {
		XCTAssertEqual(SUT(exactly: SUT.sample.bytes), SUT.sample)
	}
	
	func test_as_signature() {
		let sut = SUT.sample
		XCTAssertEqual(sut.signature, Signature.secp256k1(value: sut))
	}
}
