import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

// MARK: - EntityProtocolTest
class EntityProtocolTest<SUT_: EntityProtocol>: EntityBaseTest<SUT_> {
	func test_extract() throws {
		func doTest(_ sut: SUT) throws {
			let embedded = sut.asGeneral
			let extracted: SUT = try embedded.extract()
			XCTAssertEqual(extracted, sut)
		}
		try SUT.sampleValues.forEach(doTest)
	}
}

// MARK: - EntityBaseTest
class EntityBaseTest<SUT_: EntityBaseProtocol>: Test<SUT_> {
	func test_network_id_of_mainnet_entities() {
		for item in SUT.sampleValuesMainnet {
			XCTAssertNoDifference(item.networkID, .mainnet)
		}
	}

	func test_network_id_of_stokenet_entities() {
		for item in SUT.sampleValuesStokenet {
			XCTAssertNoDifference(item.networkID, .stokenet)
		}
	}

	func test_id_is_address() {
		for sampleValue in SUT.sampleValues {
			XCTAssertNoDifference(sampleValue.id, sampleValue.address)
		}
	}

	func test_is_hidden() {
		XCTAssertFalse(SUT.sample.isHidden)
	}

	func test_hasAuthenticationSigningKey() {
		func doTest(_ sut: SUT) {
			XCTAssertFalse(sut.hasAuthenticationSigningKey)
		}
		SUT.sampleValues.forEach(doTest)
	}

	func test_deviceFactorSourceID() {
		func doTest(_ sut: SUT) {
			XCTAssertTrue(
				sut.virtualHierarchicalDeterministicFactorInstances
					.map(\.factorSourceID)
					.contains(sut.deviceFactorSourceID!)
			)
			XCTAssertTrue(
				sut.asGeneral.virtualHierarchicalDeterministicFactorInstances
					.map(\.factorSourceID)
					.contains(sut.deviceFactorSourceID!)
			)
		}
		SUT.sampleValues.forEach(doTest)
	}

	func test_controlled_by_ed25519_factor() {
		for sampleValue in SUT.sampleValues {
			switch sampleValue.securityState {
			case let .unsecured(unsecuredEntityControl):
				switch unsecuredEntityControl.transactionSigning.publicKey.publicKey {
				case .ed25519: break // good
				case .secp256k1: XCTFail("Wrong key kind")
				}
			}
		}
	}

	func test_all_address_different() {
		XCTAssertGreaterThanOrEqual(Set(SUT.sampleValues).count, 6)
	}

	func test_flags() {
		XCTAssertTrue(
			SUT.sampleValues.flatMap(
				\.flags
			).contains(.deletedByUser)
		)
	}
}
