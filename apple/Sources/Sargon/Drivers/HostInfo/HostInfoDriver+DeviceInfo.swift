import Foundation
import SargonUniFFI

// MARK: - AppleHostInfoDriver
/// An `HostInfoDriver` actor being able to read host info, i.e
/// details about the iPhone the app is running on.
public final actor AppleHostInfoDriver {
	private var appVersion: String
	public init(appVersion: String) {
		self.appVersion = appVersion
	}
}

extension AppleHostInfoDriver {
	public nonisolated func hostAppVersion() async -> String {
		await self.appVersion
	}
}

#if canImport(UIKit)
import UIKit

extension AppleHostInfoDriver: HostInfoDriver {
	public func hostOs() async -> HostOs {
		await HostOs.ios(version: UIDevice.current.systemVersion)
	}

	public nonisolated func hostDeviceName() async -> String {
		await UIDevice.current.name
	}

	public nonisolated func hostDeviceSystemVersion() async -> String {
		await UIDevice.current.systemVersion
	}

	public nonisolated func hostDeviceModel() async -> String {
		await UIDevice.current.model
	}
}
#else

extension AppleHostInfoDriver: HostInfoDriver {
	public func hostOs() async -> HostOs {
		let info = ProcessInfo.processInfo.operatingSystemVersion
		let version = "\(info.majorVersion).\(info.minorVersion).\(info.patchVersion)"
		return HostOs.ios(version: version)
	}

	public nonisolated func hostDeviceModel() async -> String {
		let service = IOServiceGetMatchingService(
			kIOMainPortDefault,
			IOServiceMatching("IOPlatformExpertDevice")
		)

		guard
			let modelData = IORegistryEntryCreateCFProperty(
				service,
				"model" as CFString,
				kCFAllocatorDefault,
				0
			)
			.takeUnretainedValue() as? Data,
			let modelString = String(data: modelData, encoding: .utf8)
		else {
			return "Unknown Model"
		}

		return modelString.trimmingCharacters(in: .controlCharacters.union(.whitespaces))
	}

	public nonisolated func hostDeviceName() async -> String {
		"Unknown Name"
	}
}
#endif // canImport(UIKit)
