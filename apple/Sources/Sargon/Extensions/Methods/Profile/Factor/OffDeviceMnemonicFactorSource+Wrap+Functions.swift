//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-06-02.
//

import Foundation
import SargonUniFFI

extension OffDeviceMnemonicFactorSource {
	public init(
		mnemonicWithPassphrase mwp: MnemonicWithPassphrase,
		hint: OffDeviceMnemonicHint
	) {
		self = newOffDeviceMnemonicFactorSourceFromMnemonicWithPassphrase(
			mwp: mwp,
			hint: hint
		)
	}
}
