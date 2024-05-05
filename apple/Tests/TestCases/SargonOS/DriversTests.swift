import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

extension Drivers {
	public static let test = Drivers(
		appVersion: "0.0.1",
		keychainService: "Test",
		userDefaultsSuite: "works.rdx"
	)
}

final class DriversTests: TestCase {
	typealias SUT = Drivers

	func test_HostInfoDriver_model() async {
		let sut = HostInfo(appVersion: "0.0.0")
		let model = await sut.hostDeviceModel()
#if os(OSX)
		XCTAssertTrue(model.lowercased().contains("mac"))
#elseif os(iOS)
		XCTAssertEqual(model, "iPhone")
#endif
	}
	
#if os(OSX)
	func test_HostInfoDriver_version() async {
		let sut = HostInfo(appVersion: "0.0.0")
		let version = await sut.hostDeviceSystemVersion()
		XCTAssertTrue(version.starts(with: "14.") || version.starts(with: "15."))
	}
#endif
}
