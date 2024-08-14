//
//  File.swift
//  
//
//  Created by Matias Bzurovski on 13/8/24.
//

import Foundation
import SargonUniFFI

extension ResourcePreferences {
	public var hiddenResources: HiddenResources {
		resourcePreferencesGetHiddenResources(resourcePreferences: self)
	}
	
	public mutating func hideResource(kind: ResourcePreferenceKind) {
		self = resourcePreferencesHideResource(resourcePreferences: self, kind: kind)
	}
	
	public mutating func unhideResource(kind: ResourcePreferenceKind) {
		self = resourcePreferencesUnhideResource(resourcePreferences: self, kind: kind)
	}
}
