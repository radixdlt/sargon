import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

final class SignedIntentTests: Test<SignedIntent> {
	func test_hash() {
		XCTAssertEqual(SUT.sample.hash().description, "signedintent_sim1ul0kjuvd63sslhxy869zdk4k3vcdg9e9244xwpuck4dyndzx9wnqrhxy5d")
	}
}
