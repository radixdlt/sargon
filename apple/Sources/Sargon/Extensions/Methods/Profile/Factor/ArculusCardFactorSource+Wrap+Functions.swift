//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-02.
//

import Foundation
import SargonUniFFI

extension ArculusCardFactorSource {

	public init(
		mnemonicWithPassphrase mwp: MnemonicWithPassphrase,
		hint: ArculusCardHint
	) {
		self = newArculusCardFactorSourceFromMnemonicWithPassphrase(
			mwp: mwp,
			hint: hint
		)
	}
	
	public init(
		mnemonicWithPassphrase mwp: MnemonicWithPassphrase,
		name: String
	) {
		self.init(mnemonicWithPassphrase: mwp, hint: .init(name: name, model: .arculusColdStorageWallet))
	}
}
