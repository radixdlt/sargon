import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AssetPreferencesTests: TestCase {
	func test_hidden_resources() {
		var sut: [ResourceAppPreference] = []
		XCTAssertTrue(sut.hiddenResources.isEmpty)

		// Hide assets
		sut.hideResource(resource: .fungible(.sample))
		sut.hideResource(resource: .nonFungible(.sample))
		sut.hideResource(resource: .fungible(.sampleOther))

		XCTAssertEqual(sut.hiddenResources, [.fungible(.sample), .nonFungible(.sample), .fungible(.sampleOther)])

		// Unhide assets
		sut.unhideResource(resource: .fungible(.sampleOther))
		sut.unhideResource(resource: .nonFungible(.sample))
		XCTAssertEqual(sut.hiddenResources, [.fungible(.sample)])
	}
}
