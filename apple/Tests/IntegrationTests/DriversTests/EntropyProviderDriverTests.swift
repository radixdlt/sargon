import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

class EntropyProviderDriverTests: DriverTest<EntropyProvider> {
	func test() {
		let sut = SUT()
		let n = 10
		XCTAssertEqual(
			Set((0 ..< n)
				.map { _ in
					sut.generateSecureRandomBytes()
				})
				.count,
			n
		)
	}
}
