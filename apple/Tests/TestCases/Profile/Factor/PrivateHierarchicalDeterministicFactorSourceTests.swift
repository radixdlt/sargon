import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class PrivateHierarchicalDeterministicFactorSourceTests: FactorSourceTest<PrivateHierarchicalDeterministicFactorSource> {
	func test_new_babylon() {
		let sut = SUT.babylon(mnemonicWithPassphrase: .sample, hostInfo: .sample)
		XCTAssertTrue(sut.supportsBabylon)
	}

	func test_new_olympia() {
		let sut = SUT.olympia(mnemonicWithPassphrase: .sample, hostInfo: .sample)
		XCTAssertTrue(sut.supportsOlympia)
	}

	func test_kind_is_device() {
		XCTAssertEqual(SUT.olympia(mnemonicWithPassphrase: .sample, hostInfo: .sample).factorSourceKind, .device)
		XCTAssertEqual(SUT.babylon(mnemonicWithPassphrase: .sample, hostInfo: .sample).factorSourceKind, .device)
	}
}
