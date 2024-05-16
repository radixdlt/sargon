import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

final class SargonOSTests: OSTest {
	typealias SUT = SargonOS
	
	override func setUp() {
		super.setUp()
		SUT._shared = nil
	}
	
	func test() async throws {
		let _ = try await SUT.boot(
			bios: .init(
				drivers: .test()
			)
		)
	}
	
	func test_set_shared() async throws {
		let sut = try await SUT.createdSharedBootingWith(
			bios: BIOS.test(
				secureStorageDriver: Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage(
					keychainService: "test"
				)
			)
		)
		XCTAssertTrue(SUT.shared === sut)
	}
	
	func test_boot_twice_throws() async throws {
		let bios = BIOS.test(
			secureStorageDriver: Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage(
				keychainService: "test"
			)
		)
		let _ = try await SUT.createdSharedBootingWith(bios: bios)
		do {
			let _ = try await SUT.createdSharedBootingWith(bios: bios, isEmulatingFreshInstall: false)
			XCTFail("Should have thrown")
		} catch let err as SargonOSAlreadyBooted {
			XCTAssertEqual(err.errorDescription, "Radix Wallet core already initialized, should not have been initialized twice. This is a Radix developer error.")
		} catch { XCTFail("Wrong error type, expected: \(SargonOSAlreadyBooted.self)") }
	}
	
	func test_boot_twice_does_not_throws_when_emulating_fresh_install() async throws {
		let bios = BIOS.test(
			secureStorageDriver: Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage(
				keychainService: "test"
			)
		)
		let first = try await SUT.createdSharedBootingWith(bios: bios)
		let second = try await SUT.createdSharedBootingWith(bios: bios, isEmulatingFreshInstall: true)
		XCTAssertFalse(first === second)
		XCTAssertTrue(SUT.shared === second)
	}
	
}

