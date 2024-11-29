import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SecurityStructureOfFactorSourcesTests: Test<SecurityStructureOfFactorSources> {
	func test_new_from_auto_in_days() {
		let sut = SUT(
			metadata: .sample,
			numberOfDaysUntilAutoConfirmation: 10,
			matrixOfFactors: .sample
		)
		XCTAssertEqual(sut.numberOfEpochsUntilAutoConfirmation, 2880)
		XCTAssertEqual(sut.metadata, .sample)
		XCTAssertEqual(sut.matrixOfFactors, .sample)
	}

	func test_id() {
		eachSample { sut in
			XCTAssertEqual(sut.id, sut.metadata.id)
		}
	}
}
