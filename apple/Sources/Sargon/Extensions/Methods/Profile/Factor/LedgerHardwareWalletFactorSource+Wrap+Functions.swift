//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-02.
//

import Foundation
import SargonUniFFI

extension LedgerHardwareWalletFactorSource {
	public init(
		mnemonicWithPassphrase mwp: MnemonicWithPassphrase,
		hint: LedgerHardwareWalletHint,
		common: FactorSourceCommon
	) {
		self = newLedgerHardwareWalletFromMnemonicWithPassphrase(
			mwp: mwp,
			hint: hint,
			common: common
		)
	}
}
