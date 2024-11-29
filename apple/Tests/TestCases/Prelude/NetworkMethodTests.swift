import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class NetworkMethodTests: Test<NetworkMethod> {
	func test_get_description() {
		XCTAssertEqual(SUT.get.description, "GET")
	}

	func test_post_description() {
		XCTAssertEqual(SUT.post.description, "POST")
	}

	func test_head_description() {
		XCTAssertEqual(SUT.head.description, "HEAD")
	}
}
