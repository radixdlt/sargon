import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class FactorSourceTest<SUT_: BaseFactorSourceProtocol>: Test<SUT_> {
	
	func test_as_general_factorSourceID() {
		eachSample { sut in
			XCTAssertEqual(sut.asGeneral.factorSourceID, sut.factorSourceID)
		}
	}
	
	func test_as_general_factorSourceKind() {
		eachSample { sut in
			XCTAssertEqual(sut.asGeneral.kind, sut.kind)
		}
	}
	
	func test_common_update() {
		eachSample { sut in
			let newDate = Date.now
			var sut = sut
			XCTAssertNotEqual(sut.lastUsedOn, newDate)
			XCTAssertNotEqual(sut.addedOn, newDate)
			sut.common.lastUsedOn = newDate
			XCTAssertEqual(sut.lastUsedOn, newDate)
		}
	}
	
	
	func test_crypto_params() {
		eachSample { sut in
			XCTAssertEqual(sut.cryptoParameters, sut.common.cryptoParameters)
			XCTAssertEqual(sut.cryptoParameters.supportsBabylon, sut.supportsBabylon)
			XCTAssertEqual(sut.cryptoParameters.supportsOlympia, sut.supportsOlympia)
		}
	}
	
	func test_flag_for_deletion() {
		eachSample { sut in
			var sut = sut
			XCTAssertFalse(sut.isFlaggedForDeletion)
			sut.flag(.deletedByUser)
			XCTAssertTrue(sut.isFlaggedForDeletion)
		}
	}
}
