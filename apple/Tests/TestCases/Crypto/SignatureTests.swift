import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SignatureTests: SignatureTest<Signature> {
	func test_signature_is_self() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut, sut.signature)
		}
		SUT.sampleValues.forEach(doTest)
	}
}
