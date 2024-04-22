import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SignatureWithPublicKeyTests: Test<SignatureWithPublicKey> {
	func test_is_valid() {
		XCTAssertFalse(SUT.sample.isValid(Hash.sample))
	}
}
