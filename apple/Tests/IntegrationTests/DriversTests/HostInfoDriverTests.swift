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

class HostInfoDriverTests: DriverTest<AppleHostInfoDriver> {
	
	func test_app_version() async throws {
		let sut = SUT()
		let info = await sut.hostAppVersion()
		XCTAssertEqual(info, appVersion)
	}
	
	func test_device_name_not_empty() async throws {
		let sut = SUT()
		let info = await sut.hostDeviceName()
		XCTAssertFalse(info.isEmpty)
	}
	
	func test_device_system_not_empty() async throws {
		let sut = SUT()
		let info = await sut.hostDeviceSystemVersion()
		XCTAssertFalse(info.isEmpty)
	}
	
	func test_device_model_not_empty() async throws {
		let sut = SUT()
		let info = await sut.hostDeviceModel()
		XCTAssertFalse(info.isEmpty)
	}
}
