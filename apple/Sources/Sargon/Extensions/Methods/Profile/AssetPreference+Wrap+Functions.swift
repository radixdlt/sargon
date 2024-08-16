//
//  File.swift
//  
//
//  Created by Matias Bzurovski on 13/8/24.
//

import Foundation
import SargonUniFFI

extension [AssetPreference] {
	public var hiddenAssets: [AssetAddress] {
		assetPreferencesGetHiddenAssets(assetPreferences: self)
	}
	
	public mutating func hideAsset(asset: AssetAddress) {
		self = assetPreferencesHideAsset(assetPreferences: self, asset: asset)
	}
	
	public mutating func unhideAsset(asset: AssetAddress) {
		self = assetPreferencesUnhideAsset(assetPreferences: self, asset: asset)
	}
}
