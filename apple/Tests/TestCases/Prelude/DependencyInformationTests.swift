import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class DependencyInformationTests: Test<DependencyInformation> {
	func test_description() {
		XCTAssertNoDifference(SUT.sample.description, "develop")
	}
}
