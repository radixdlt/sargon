//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI

public final actor HostInfo {
	fileprivate var appVersion: String?
	private init() {
		appVersion = nil
	}
	public static let shared = HostInfo()
}

extension HostInfo {
	public func setAppVersion(appVersion: String) {
		self.appVersion = appVersion
	}
	
	nonisolated public func hostAppVersion() async -> String {
		await self.appVersion ?? "NOT SET"
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
	nonisolated public func hostDeviceSystemVersion() async -> String {
		"HARDCODED macOS unknown version"
	}
	
	nonisolated public func hostDeviceModel() async -> String {
		"HARDCODED macOS unknown computer model"
	}
	
	nonisolated public func hostDeviceName() async -> String {
		"HARDCODED macOS unknown computer name"
	}
}
#endif // canImport(UIKit)
