import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class NonceTests: Test<Nonce> {
	/// with very low probability this will fail
	func test_secure_random() {
		let n = 10
		let sut = Set<SUT>(
			(0 ..< n).map { _ in SUT.secureRandom() }
		)
		XCTAssertEqual(sut.count, n)
		XCTAssertEqual(Set(sut.map(\.value)).count, n)
	}
}
