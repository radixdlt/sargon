import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class BIOSTests: OSTest {
	typealias SUT = BIOS
	
	func test_set_shared() {
		let sut = SUT.createdShared(bundle: .main, userDefaultsSuite: "test", secureStorageDriver: Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage(keychainService: "test"))
		XCTAssertTrue(SUT.shared === sut)
		let new = SUT.settingShared(shared: .test(secureStorageDriver: Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage.init(keychainService: "other")), isEmulatingFreshInstall: true)
		
		XCTAssertFalse(sut === new)
		XCTAssertTrue(SUT.shared === new)
	}
}
