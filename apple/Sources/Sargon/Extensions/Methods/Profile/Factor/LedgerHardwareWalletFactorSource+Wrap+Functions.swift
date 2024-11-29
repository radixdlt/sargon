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
