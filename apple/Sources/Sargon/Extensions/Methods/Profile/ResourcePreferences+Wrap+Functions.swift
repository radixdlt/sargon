//
//  File.swift
//  
//
//  Created by Matias Bzurovski on 13/8/24.
//

import Foundation
import SargonUniFFI

extension ResourcePreferences {
	public var hiddenResources: [ResourceAddress] {
		resourcePreferencesGetHiddenResources(resourcePreferences: self)
	}
	
	public func hasResourceHidden(resource: ResourceAddress) -> Bool {
		resourcePreferencesHasResourceHidden(resourcePreferences: self, resource: resource)
	}
	
	public mutating func hideResource(resource: ResourceAddress) {
		self = resourcePreferencesHideResource(resourcePreferences: self, resource: resource)
	}
	
	public mutating func unhideResource(resource: ResourceAddress) {
		self = resourcePreferencesUnhideResource(resourcePreferences: self, resource: resource)
	}
}
