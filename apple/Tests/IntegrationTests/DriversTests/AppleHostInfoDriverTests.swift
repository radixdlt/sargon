import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

private let appVersion = "0.0.0"
extension AppleHostInfoDriver {
	init() {
		self.init(appVersion: appVersion)
	}
}

// MARK: - AppleHostInfoDriverTests
class AppleHostInfoDriverTests: DriverTest<AppleHostInfoDriver> {
	func test_app_version() async {
		let sut = SUT()
		let info = await sut.hostAppVersion()
		XCTAssertEqual(info, appVersion)
	}

	func test_device_name_not_empty() async {
		let sut = SUT()
		let info = await sut.hostDeviceName()
		XCTAssertFalse(info.isEmpty)
	}

	func test_device_os_name_not_empty() async {
		let sut = SUT()
		let info = await sut.hostOs()
		XCTAssertFalse(info.name().isEmpty)
	}

	func test_device_model_not_empty() async {
		let sut = SUT()
		let info = await sut.hostDeviceModel()
		XCTAssertFalse(info.isEmpty)
	}
}
