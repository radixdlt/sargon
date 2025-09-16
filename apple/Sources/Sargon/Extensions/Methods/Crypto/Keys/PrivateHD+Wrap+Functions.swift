import Foundation
import SargonUniFFI

extension PrivateHierarchicalDeterministicFactorSource {
	public static func olympia(
		mnemonicWithPassphrase: MnemonicWithPassphrase,
		hostInfo: HostInfo
	) -> Self {
		newPrivateHdFactorSourceOlympiaFromMnemonicWithPassphrase(
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			hostInfo: hostInfo
		)
	}

	public static func babylon(
		mnemonicWithPassphrase: MnemonicWithPassphrase,
		hostInfo: HostInfo
	) -> Self {
		newPrivateHdFactorSourceBabylonFromMnemonicWithPassphrase(
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			hostInfo: hostInfo
		)
	}
}
