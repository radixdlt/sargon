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
