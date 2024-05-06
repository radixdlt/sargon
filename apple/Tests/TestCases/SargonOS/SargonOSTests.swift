import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SargonOSTests: TestCase {
	typealias SUT = SargonOS
	
	func test() async throws {
		let _ = try await SUT.boot(
			bios: .init(
				drivers: .test
			)
		)
	}
}

