import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class OSTest: TestCase {
	
	override class func shouldEnableRustLog() -> Bool {
		// BIOS and SargonOS tests will have enabled Rust logging from inside of rust.
		// we should not enable it twice which will lead to crash.
		false
	}
	
}
