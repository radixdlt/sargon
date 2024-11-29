import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

final class BlobTests: Test<Blob> {
	func test_blob() {
		let data = Data.random(byteCount: 16)
		let blob = SUT(data: data)
		XCTAssertEqual(blob.data, data)
		XCTAssertEqual(blob.hex, data.hex)
	}
}
