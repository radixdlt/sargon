import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class ResourceAddressTests: AddressTest<ResourceAddress> {
	func test_is_fungible() {
		XCTAssertTrue(SUT.sampleMainnetXRD.isFungible)
		XCTAssertFalse(SUT.sampleMainnetNonFungibleGCMembership.isFungible)
	}

	func test_is_non_fungible() {
		XCTAssertFalse(SUT.sampleMainnetXRD.isNonFungible)
		XCTAssertTrue(SUT.sampleMainnetNonFungibleGCMembership.isNonFungible)
	}

	func test_as_non_fungible() {
		XCTAssertNotNil(SUT.sampleMainnetNonFungibleGCMembership.asNonFungibleResourceAddress)
	}

	func test_xrd_on_network() throws {
		XCTAssertEqual(SUT.xrd(on: .mainnet), SUT.sampleMainnet)
		XCTAssertEqual(SUT.xrd(on: .stokenet), SUT.sampleStokenet)
		XCTAssertEqual(
			SUT.xrd(on: .simulator),
			try SUT(
				validatingAddress: "resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3"
			)
		)
		XCTAssertEqual(SUT.sampleMainnetXRD.xrdOnSameNetwork, SUT.sampleMainnetXRD)
		XCTAssertEqual(SUT.sampleStokenetXRD.xrdOnSameNetwork, SUT.sampleStokenetXRD)

		XCTAssertEqual(AccountAddress.sampleMainnet.xrdOnSameNetwork, SUT.sampleMainnetXRD)
		XCTAssertEqual(AccountAddress.sampleMainnetOther.xrdOnSameNetwork, SUT.sampleMainnetXRD)
		XCTAssertEqual(AccountAddress.sampleStokenet.xrdOnSameNetwork, SUT.sampleStokenetXRD)
		XCTAssertEqual(AccountAddress.sampleStokenetOther.xrdOnSameNetwork, SUT.sampleStokenetXRD)

		XCTAssertEqual(IdentityAddress.sampleMainnet.xrdOnSameNetwork, SUT.sampleMainnetXRD)
		XCTAssertEqual(IdentityAddress.sampleMainnetOther.xrdOnSameNetwork, SUT.sampleMainnetXRD)
		XCTAssertEqual(IdentityAddress.sampleStokenet.xrdOnSameNetwork, SUT.sampleStokenetXRD)
		XCTAssertEqual(IdentityAddress.sampleStokenetOther.xrdOnSameNetwork, SUT.sampleStokenetXRD)
	}

	func test_is_xrd() {
		XCTAssertTrue(SUT.sampleMainnetXRD.isXRD)
		XCTAssertTrue(SUT.sampleStokenetXRD.isXRD)

		XCTAssertFalse(SUT.sampleMainnetCandy.isXRD)
		XCTAssertFalse(SUT.sampleMainnetNonFungibleGCMembership.isXRD)
		XCTAssertFalse(SUT.sampleStokenetGum.isXRD)
		XCTAssertFalse(SUT.sampleStokenetGC.isXRD)
	}

	func test_is_xrd_on_network() {
		XCTAssertTrue(SUT.sampleMainnet.isXRD(on: .mainnet))
		XCTAssertFalse(SUT.sampleMainnet.isXRD(on: .stokenet))

		XCTAssertFalse(SUT.sampleStokenet.isXRD(on: .mainnet))
		XCTAssertTrue(SUT.sampleStokenet.isXRD(on: .stokenet))
	}

	func test_xrd_of_mainnet() {
		XCTAssertEqual(SUT.mainnetXRD, SUT.sampleMainnet)
		XCTAssert(SUT.mainnetXRD.isXRD)
		XCTAssert(SUT.mainnetXRD.isXRD(on: .mainnet))
		XCTAssertFalse(SUT.mainnetXRD.isXRD(on: .stokenet))
	}
}
