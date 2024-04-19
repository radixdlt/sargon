import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class ResourceIndicatorTests: Test<ResourceIndicator> {
	func test_resource_address() {
		let sut = SUT.sample
		switch sut {
		case let .fungible(resourceAddress, _):
			XCTAssertEqual(resourceAddress, sut.resourceAddress)
		case .nonFungible: XCTFail("Expected fungible")
		}
	}
}
