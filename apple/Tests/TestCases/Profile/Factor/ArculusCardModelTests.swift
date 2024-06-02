import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class ArculusCardModelTests: TestCase {
	typealias SUT = ArculusCardModel
	
	func test_description() {
		XCTAssertEqual(SUT.arculusColdStorageWallet.description, "arculusColdStorageWallet")
	}
}
