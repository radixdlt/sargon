import Foundation
import SargonUniFFI

extension [ResourceAppPreference] {
	public var hiddenResources: [ResourceIdentifier] {
		resourcePreferencesGetHiddenResources(resourcePreferences: self)
	}

	public mutating func hideResource(resource: ResourceIdentifier) {
		self = resourcePreferencesHideResource(resourcePreferences: self, resource: resource)
	}

	public mutating func unhideResource(resource: ResourceIdentifier) {
		self = resourcePreferencesUnhideResource(resourcePreferences: self, resource: resource)
	}
}
