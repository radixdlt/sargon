//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-06-02.
//

import Foundation
import SargonUniFFI

extension PasswordFactorSource {

	public init(
		mnemonicWithPassphrase mwp: MnemonicWithPassphrase,
		hint: PasswordFactorSourceHint
	) {
		self = newPasswordFactorSourceFromMnemonicWithPassphrase(
			mwp: mwp,
			hint: hint
		)
	}
}
