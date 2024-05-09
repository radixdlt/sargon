import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class FactorSourceTest<SUT_: BaseFactorSourceProtocol>: Test<SUT_> {
	
	func test_as_general_factorSourceID() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.asGeneral.factorSourceID, sut.factorSourceID)
		}
		SUT.sampleValues.forEach(doTest)
	}
	
	func test_as_general_factorSourceKind() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.asGeneral.kind, sut.kind)
		}
		SUT.sampleValues.forEach(doTest)
	}
	
	func test_common_update() {
		func doTest(_ sut: SUT) {
			let newDate = Date.now
			var sut = sut
			XCTAssertNotEqual(sut.lastUsedOn, newDate)
			XCTAssertNotEqual(sut.addedOn, newDate)
			sut.common.lastUsedOn = newDate
			XCTAssertEqual(sut.lastUsedOn, newDate)
		}
		SUT.sampleValues.forEach(doTest)
	}
	
	
	func test_crypto_params() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.cryptoParameters, sut.common.cryptoParameters)
			XCTAssertEqual(sut.cryptoParameters.supportsBabylon, sut.supportsBabylon)
			XCTAssertEqual(sut.cryptoParameters.supportsOlympia, sut.supportsOlympia)
		}
		SUT.sampleValues.forEach(doTest)
	}
	
	func test_flag_for_deletion() {
		func doTest(_ sut: SUT) {
			var sut = sut
			XCTAssertFalse(sut.isFlaggedForDeletion)
			sut.flag(.deletedByUser)
			XCTAssertTrue(sut.isFlaggedForDeletion)
		}
		SUT.sampleValues.forEach(doTest)
	}
}
