import Foundation
import SargonUniFFI

extension PassphraseFactorSource {
	public init(
		mnemonicWithPassphrase mwp: MnemonicWithPassphrase
	) {
		self = newPassphraseFactorSourceFromMnemonicWithPassphrase(mwp: mwp)
	}
}
