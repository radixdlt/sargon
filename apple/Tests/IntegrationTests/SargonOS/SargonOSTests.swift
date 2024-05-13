import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SargonOSTests: OSTest {
	typealias SUT = SargonOS
	
	func test() async throws {
		let _ = try await SUT.boot(
			bios: .init(
				drivers: .test()
			)
		)
	}
}

