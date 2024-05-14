//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI

public final actor HostInfo {
	fileprivate var appVersion: String
	public init(appVersion: String) {
		self.appVersion = appVersion
	}
}

extension HostInfo {
	nonisolated public func hostAppVersion() async -> String {
		await self.appVersion
	}
}

#if canImport(UIKit)
import UIKit
extension HostInfo: HostInfoDriver {
	nonisolated public func hostDeviceName() async -> String {
		await UIDevice.current.name
	}
	
	nonisolated public func hostDeviceSystemVersion() async -> String {
		await UIDevice.current.systemVersion
	}
	
	nonisolated public func hostDeviceModel() async -> String {
		await UIDevice.current.model
	}
}
#else

extension HostInfo: HostInfoDriver {
	
	/// We cannot read a stable device if on iOS. We return `nil` so that Rust Sargon can generate
	/// and save a device identifier for us.
	public func hostDeviceId() async -> DeviceId? {
		nil
	}
	
	nonisolated public func hostDeviceSystemVersion() async -> String {
		let info = ProcessInfo.processInfo.operatingSystemVersion
		return "\(info.majorVersion).\(info.minorVersion).\(info.patchVersion)"
	}
	
	nonisolated public func hostDeviceModel() async -> String {
	
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
	
	nonisolated public func hostDeviceName() async -> String {
		"Unknown Name"
	}
}
#endif // canImport(UIKit)
