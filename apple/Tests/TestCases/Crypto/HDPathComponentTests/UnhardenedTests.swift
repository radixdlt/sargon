import Sargon
import XCTest

final class UnhardenedTests: HDPathComponentProtocolTest<Unhardened> {
	func test_fromU31() throws {
		let sut = try SUT(u31: U31(value: 5))
		try XCTAssertEqual(SUT(localKeySpace: 5), sut)
	}
}
