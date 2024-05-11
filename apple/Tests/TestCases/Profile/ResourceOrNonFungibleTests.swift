import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class ResourceOrNonFungibleTests: Test<ResourceOrNonFungible> {
	func test_resource_address_fungible() {
		let resourceAddress = ResourceAddress.sampleMainnetCandy
		let sut = SUT.resource(value: resourceAddress)
		XCTAssertEqual(sut.resourceAddress, resourceAddress)
	}
	func test_resource_address_non_fungible() {
		let resourceAddress = NonFungibleResourceAddress.sample.asResourceAddress
		let globalID = NonFungibleGlobalID(resourceAddress: resourceAddress, nonFungibleLocalId: 1)
		let sut = SUT.nonFungible(value: globalID)
		XCTAssertEqual(sut.resourceAddress, resourceAddress)
	}
	func test_id_is_self() {
		eachSample { sut in
			XCTAssertEqual(sut.id, sut)
		}
	}
}
