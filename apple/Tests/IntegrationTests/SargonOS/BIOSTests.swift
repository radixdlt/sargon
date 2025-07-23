import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

extension BIOS {
	static func creatingShared(
		bundle: Bundle = .main,
		userDefaultsSuite: String = "test",
		unsafeStorageKeyMapping: UnsafeStorageKeyMapping = [:],
		secureStorageDriver: SecureStorageDriver = Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage(
			keychainService: "test"
		)
	) -> BIOS {
		creatingShared(drivers: .init(bundle: bundle, userDefaultsSuite: userDefaultsSuite, unsafeStorageKeyMapping: unsafeStorageKeyMapping, secureStorageDriver: secureStorageDriver, arculuCSDKDriver: ArculusCsdkDriverImpl(noPointer: .init()), nftTagDriver: NfcTagDriverImpl(noPointer: .init())))
	}
}

// MARK: - BIOSTests
final class BIOSTests: OSTest {
	typealias SUT = BIOS

	func test_set_shared() {
		let sut = SUT.creatingShared()

		XCTAssertTrue(SUT.shared === sut)
		let new = SUT.settingShared(
			shared: .test(
				secureStorageDriver: Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage(
					keychainService: "other"
				)
			),
			isEmulatingFreshInstall: true
		)

		XCTAssertFalse(sut === new)
		XCTAssertTrue(SUT.shared === new)
	}
}
