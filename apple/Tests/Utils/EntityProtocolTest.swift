import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class EntityProtocolTest<SUT_: EntityProtocol>: EntityBaseTest<SUT_> {
	
	func test_extract() throws {
		try eachSample { sut in
			let embedded = sut.asGeneral
			let extracted: SUT = try embedded.extract()
			XCTAssertEqual(extracted, sut)
		}
	}
}


class EntityBaseTest<SUT_: EntityBaseProtocol>: Test<SUT_> {
	
	func test_network_id_of_mainnet_entities() {
		SUT.sampleValuesMainnet.forEach {
			XCTAssertNoDifference($0.networkID, .mainnet)
		}
	}
	
	func test_network_id_of_stokenet_entities() {
		SUT.sampleValuesStokenet.forEach {
			XCTAssertNoDifference($0.networkID, .stokenet)
		}
	}
	
	func test_id_is_address() {
		SUT.sampleValues.forEach {
			XCTAssertNoDifference($0.id, $0.address)
		}
	}
	
	func test_is_hidden() {
		XCTAssertFalse(SUT.sample.isHidden)
	}
	
	func test_hasAuthenticationSigningKey() {
		eachSample { sut in
			XCTAssertFalse(sut.hasAuthenticationSigningKey)
		}
	}
	
	func test_deviceFactorSourceID() {
		eachSample { sut in
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
	}

	func test_controlled_by_ed25519_factor() {
		SUT.sampleValues.forEach {
            switch $0.securityState {
            case .unsecured(let unsecuredEntityControl):
                switch 	unsecuredEntityControl.transactionSigning.publicKey.publicKey {
                case .ed25519: break // good
                case .secp256k1: XCTFail("Wrong key kind")
                }
                // TODO: Handle
                //			case .securified(value: _):
                //				XCTFail("Wrong security state")
                //			}
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
			).contains(.hiddenByUser)
		)
	}
}
