import Foundation
import SargonUniFFI

extension DeviceFactorSource {
	public static func olympia(
		mnemonicWithPassphrase: MnemonicWithPassphrase
	) -> Self {
		newDeviceFactorSourceOlympia(
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			walletClientModel: .iphone
		)
	}

	public static func babylon(
		mnemonicWithPassphrase: MnemonicWithPassphrase,
		isMain: Bool
	) -> Self {
		newDeviceFactorSourceBabylon(
			isMain: isMain,
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			walletClientModel: .iphone
		)
	}

	public var isMainBDFS: Bool {
		deviceFactorSourceIsMainBdfs(deviceFactorSource: self)
	}
}
