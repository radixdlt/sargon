import Foundation
import SargonUniFFI

// MARK: - AppPreferences + SargonModel
extension AppPreferences: SargonModel {}

extension AppPreferences {
	public func hasGateway(with url: FfiUrl) -> Bool {
		appPreferencesHasGatewayWithUrl(appPreferences: self, url: url)
	}
}
