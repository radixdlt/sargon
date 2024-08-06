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
	public func hasGateway(with url: FfiUrl) -> Bool {
		appPreferencesHasGatewayWithUrl(appPreferences: self, url: url)
	}
}
