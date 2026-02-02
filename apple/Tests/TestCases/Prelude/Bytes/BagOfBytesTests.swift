import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

final class BagOfBytesTests: XCTestCase {
	func test_equality() {
		XCTAssertNoDifference(Data.sampleDead, Data.sampleDead)
		XCTAssertNoDifference(Data.sampleCafe, Data.sampleCafe)
	}

	func test_inequality() {
		XCTAssertNotEqual(Data.sampleCafe, Data.sampleDead)
	}

	func test_new_bag_of_bytes_dead() throws {
		let hex = String(repeating: "dead", count: 16)
		let data = try Data(hex: hex)
		XCTAssertNoDifference(newBagOfBytesFrom(bytes: data), Data.sampleDead)
	}

	func test_new_bag_of_bytes_cafe() throws {
		let hex = String(repeating: "cafe", count: 16)
		let data = try Data(hex: hex)
		XCTAssertNoDifference(newBagOfBytesFrom(bytes: data), Data.sampleCafe)
	}
}
