import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SavedGatewaysTests: Test<SavedGateways> {
	func test_preset_is_default() {
		XCTAssertEqual(SUT.preset, SUT.default)
	}

	func test_init_current_other() {
		XCTAssertEqual(SUT(current: Gateway.mainnet, other: [Gateway.stokenet]), SUT.preset)
	}

	func test_init_current_only() {
		let sut = SUT(current: Gateway.mainnet)
		XCTAssertEqual(sut.current, .mainnet)
		XCTAssertEqual(sut.other, [])
	}

	func test_change_current_to() throws {
		var sut = SUT(current: .stokenet, other: [.mainnet])
		try sut.changeCurrent(to: .mainnet)
		XCTAssertEqual(sut, .preset)
	}

	func test_all() {
		XCTAssertEqual(SUT.preset.all, [.mainnet, .stokenet])
	}
}
