//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-06-02.
//

import Foundation
import SargonUniFFI

extension PassphraseFactorSource {

	public init(
		mnemonicWithPassphrase mwp: MnemonicWithPassphrase
	) {
		self = newPassphraseFactorSourceFromMnemonicWithPassphrase(mwp: mwp)
	}
}
