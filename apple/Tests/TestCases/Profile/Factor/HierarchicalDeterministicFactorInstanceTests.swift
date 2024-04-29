import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class HierarchicalDeterministicFactorInstanceTests: Test<HierarchicalDeterministicFactorInstance> {
	func test_factorSourceID() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.factorSourceID, sut.factorSourceId)
		}
		SUT.sampleValues.forEach(doTest)
	}

	func test_derivationPath() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.derivationPath, sut.publicKey.derivationPath)
		}
		SUT.sampleValues.forEach(doTest)
	}

	func test_factor_instance() {
		func doTest(_ sut: SUT) {
			let factor = sut.factorInstance
			XCTAssertEqual(factor.factorSourceId, sut.factorSourceId.asGeneral)
			switch factor.badge {
			case let .virtual(.hierarchicalDeterministic(hdKey)):
				XCTAssertEqual(hdKey.derivationPath, sut.derivationPath)
				XCTAssertEqual(hdKey.publicKey, sut.publicKey.publicKey)
			}
		}
		SUT.sampleValues.forEach(doTest)
	}
}
