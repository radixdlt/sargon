import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FfiUrlTests: TestCase {
	func test_url() throws {
		let url = try XCTUnwrap(URL(string: "https://radixdlt.com/"))
		let sut = try FfiUrl(urlPath: url.absoluteString)
		XCTAssertEqual(sut.url, url)
	}
}
