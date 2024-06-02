//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-02.
//

import Foundation
import SargonUniFFI

extension ArculusCardFactorSource {
	public init(mnemonicWithPassphrase mwp: MnemonicWithPassphrase, hint: ArculusCardHint) {
		self = newArculusCardFactorSourceFromMnemonicWithPassphrase(
			mwp: mwp,
			hint: hint
		)
	}
}
