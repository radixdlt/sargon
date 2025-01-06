import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceIDTests: FactorSourceIDTest<FactorSourceID> {
	func test_as_general_is_self() {
		eachSample { sut in
			XCTAssertEqual(sut.asGeneral, sut)
		}
	}

	func test_extract_wrong_throws() throws {
    	try eachSample { sut in
    		XCTAssertThrowsError(try sut.asGeneral.extract(as: FactorSourceID.self))
    	}
    }
}
