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
		label: String
	) {
		self.init(mnemonicWithPassphrase: mwp, hint: .init(label: label, model: .arculusColdStorageWallet))
	}
}
