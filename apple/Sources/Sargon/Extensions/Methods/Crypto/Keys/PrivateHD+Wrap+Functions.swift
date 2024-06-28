//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

extension PrivateHierarchicalDeterministicFactorSource {
	
	public static func olympia(
		mnemonicWithPassphrase: MnemonicWithPassphrase,
		deviceInfo: DeviceInfo
	) -> Self {
		newPrivateHdFactorSourceOlympiaFromMnemonicWithPassphrase(
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			deviceInfo: deviceInfo
		)
	}
	
	public static func babylon(
		isMainBDFS: Bool,
		mnemonicWithPassphrase: MnemonicWithPassphrase,
		deviceInfo: DeviceInfo
	) -> Self {
		newPrivateHdFactorSourceBabylonFromMnemonicWithPassphrase(
			isMain: isMainBDFS,
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			deviceInfo: deviceInfo
		)
	}
}
