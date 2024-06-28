//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

extension DeviceFactorSource {

	public static func olympia(
		mnemonicWithPassphrase: MnemonicWithPassphrase,
		deviceInfo: DeviceInfo
	) -> Self {
		newDeviceFactorSourceOlympia(
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			deviceInfo: deviceInfo
		)
	}
	
	public static func babylon(
		mnemonicWithPassphrase: MnemonicWithPassphrase,
		isMain: Bool,
		deviceInfo: DeviceInfo
	) -> Self {
		newDeviceFactorSourceBabylon(
			isMain: isMain,
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			deviceInfo: deviceInfo
		)
	}
	
	
	public var isMainBDFS: Bool {
		deviceFactorSourceIsMainBdfs(deviceFactorSource: self)
	}
}
