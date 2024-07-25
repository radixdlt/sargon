//
//  File.swift
//  
//
//  Created by Michael Bakogiannis on 24/7/24.
//

import Foundation

extension HostOs: SargonModel {}

extension HostOs {
	public static func ios(version: String) -> HostOs {
		newHostOsIos(version: version)
	}

	public func name() -> String {
		hostOsGetName(hostOs: self)
	}

	public func version() -> String {
		hostOsGetVersion(hostOs: self)
	}

	public func vendor() -> String {
		hostOsGetVendor(hostOs: self)
	}
}
