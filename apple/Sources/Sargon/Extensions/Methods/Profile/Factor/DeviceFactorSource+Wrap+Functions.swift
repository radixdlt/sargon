import Foundation
import SargonUniFFI

extension DeviceFactorSource {
	public static func olympia(
		mnemonicWithPassphrase: MnemonicWithPassphrase,
		hostInfo: HostInfo
	) -> Self {
		newDeviceFactorSourceOlympia(
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			hostInfo: hostInfo
		)
	}

	public static func babylon(
		mnemonicWithPassphrase: MnemonicWithPassphrase,
		hostInfo: HostInfo
	) -> Self {
		newDeviceFactorSourceBabylon(
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			hostInfo: hostInfo
		)
	}
}
