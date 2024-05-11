import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SignatureTests: SignatureTest<Signature> {
	func test_signature_is_self() {
		eachSample { sut in
			XCTAssertEqual(sut, sut.signature)
		}
	}
}
