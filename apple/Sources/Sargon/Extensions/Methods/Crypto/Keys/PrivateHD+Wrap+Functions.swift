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
		mnemonicWithPassphrase: MnemonicWithPassphrase
	) -> Self {
		newPrivateHdFactorSourceOlympiaFromMnemonicWithPassphrase(
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			walletClientModel: .iphone
		)
	}
	
	public static func babylon(
		isMainBDFS: Bool,
		mnemonicWithPassphrase: MnemonicWithPassphrase
	) -> Self {
		newPrivateHdFactorSourceBabylonFromMnemonicWithPassphrase(
			isMain: isMainBDFS,
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			walletClientModel: .iphone
		)
	}
}
