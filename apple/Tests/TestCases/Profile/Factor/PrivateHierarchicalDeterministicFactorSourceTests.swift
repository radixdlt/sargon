import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class PrivateHierarchicalDeterministicFactorSourceTests: FactorSourceTest<PrivateHierarchicalDeterministicFactorSource> {

	func test_new_babylon() {
		let sut = SUT.babylon(isMainBDFS: true, mnemonicWithPassphrase: .sample, hostInfo: .sample)
		XCTAssertTrue(sut.supportsBabylon)
	}
	
	func test_new_olympia() {
		let sut = SUT.olympia(mnemonicWithPassphrase: .sample, hostInfo: .sample)
		XCTAssertTrue(sut.supportsOlympia)
	}
	
	func test_kind_is_device() {
		XCTAssertEqual(SUT.olympia(mnemonicWithPassphrase: .sample, hostInfo: .sample).factorSourceKind, .device)
		XCTAssertEqual(SUT.babylon(isMainBDFS: true, mnemonicWithPassphrase: .sample, hostInfo: .sample).factorSourceKind, .device)
	}
	
	func test_is_main_bdfs_true() {
		let sut = SUT.babylon(isMainBDFS: true, mnemonicWithPassphrase: .sample, hostInfo: .sample)
		XCTAssertTrue(sut.factorSource.isMainBDFS)
	}
	
	func test_is_main_bdfs_false() {
		let sut = SUT.babylon(isMainBDFS: false, mnemonicWithPassphrase: .sample, hostInfo: .sample)
		XCTAssertFalse(sut.factorSource.isMainBDFS)
	}
}
