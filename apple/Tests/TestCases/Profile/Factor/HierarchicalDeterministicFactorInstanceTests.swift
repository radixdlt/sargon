import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class HierarchicalDeterministicFactorInstanceTests: Test<HierarchicalDeterministicFactorInstance> {
	
	func test_factorSourceID() {
		eachSample { sut in
			XCTAssertEqual(sut.factorSourceID, sut.factorSourceId)
		}
	}
	
	func test_derivationPath() {
		eachSample { sut in
			XCTAssertEqual(sut.derivationPath, sut.publicKey.derivationPath)
		}
	}
	
	func test_factor_instance() {
		eachSample { sut in
			let factor = sut.factorInstance
			XCTAssertEqual(factor.factorSourceId, sut.factorSourceId.asGeneral)
			switch factor.badge {
			case let .virtual(.hierarchicalDeterministic(hdKey)):
				XCTAssertEqual(hdKey.derivationPath, sut.derivationPath)
				XCTAssertEqual(hdKey.publicKey, sut.publicKey.publicKey)
			case .physical: XCTFail("not supported yet")
			}
		}
	}
}
