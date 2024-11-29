import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SargonBuildInformationTests: Test<SargonBuildInformation> {
	func test_build_information() {
		let info = SargonBuildInformation.get()
		XCTAssert(info.sargonVersion.contains("."))
		XCTAssertFalse(String(describing: info.dependencies.radixEngineToolkit).isEmpty)
		XCTAssertFalse(String(describing: info.dependencies.scryptoRadixEngine).isEmpty)
	}
}
