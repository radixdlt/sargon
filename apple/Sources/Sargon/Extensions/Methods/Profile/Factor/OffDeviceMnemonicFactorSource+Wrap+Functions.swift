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
