import Sargon
import XCTest

final class ProfileTests: XCTestCase {
	func test_equatable() {
		let p = Profile.sample
		let q = Profile.sampleOther
		XCTAssertEqual(
			p, Profile.sample
		)
		XCTAssertNotEqual(p, q)
		XCTAssertEqual(q, Profile.sampleOther)
	}

	func test_hashable() {
		let a = Profile.sample
		let b = Profile.sampleOther
		XCTAssertEqual(Set([a, a]).count, 1)
		XCTAssertEqual(Set([b, b]).count, 1)
		XCTAssertEqual(Set([a, b, b, a]).count, 2)
	}
}
