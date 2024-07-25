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
		hostInfo: HostInfo
	) -> Self {
		newDeviceFactorSourceOlympia(
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			hostInfo: hostInfo
		)
	}
	
	public static func babylon(
		mnemonicWithPassphrase: MnemonicWithPassphrase,
		isMain: Bool,
		hostInfo: HostInfo
	) -> Self {
		newDeviceFactorSourceBabylon(
			isMain: isMain,
			mnemonicWithPassphrase: mnemonicWithPassphrase,
			hostInfo: hostInfo
		)
	}
	
	
	public var isMainBDFS: Bool {
		deviceFactorSourceIsMainBdfs(deviceFactorSource: self)
	}
}
