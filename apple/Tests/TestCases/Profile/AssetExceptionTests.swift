import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AssetExceptionTests: Test<AssetException> {
	func test_id_is_address() {
		eachSample { sut in
			XCTAssertEqual(sut.id, sut.address)
		}
	}
}
