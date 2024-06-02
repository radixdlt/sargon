import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class SpecificFactorSourceTest<SUT_: FactorSourceProtocol>: FactorSourceTest<SUT_> {
	
	func test_get_set_common() {
		let samples = SUT.sampleValues.map {
			var factor = $0.asGeneral
			factor.common.lastUsedOn = .init(timeIntervalSince1970: 0)
			factor.common.addedOn = factor.common.lastUsedOn
			return factor
		}
		XCTAssertTrue(samples.allSatisfy({ $0.common.addedOn == .init(timeIntervalSince1970: 0) }))
	}
	
	func test_extract() throws {
		try eachSample { sut in
			let embedded = sut.asGeneral
			let extracted: SUT = try embedded.extract()
			XCTAssertEqual(extracted, sut)
		}
	}
}
