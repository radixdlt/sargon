//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

extension AppPreferences: SargonModel {}

extension AppPreferences {
	public func hasGateway(with url: URL) throws -> Bool {
		let ffiUrl = try FfiUrl(url: url)
		return appPreferencesHasGatewayWithUrl(appPreferences: self, url: ffiUrl)
	}
}
