//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-18.
//

import Foundation
import SargonUniFFI

extension DeviceInfo {
	public static func iPhone() -> Self {
		newDeviceInfoIphone()
	}
	
	public init(jsonData: some DataProtocol) throws {
		self = try newDeviceInfoFromJsonBytes(jsonBytes: Data(jsonData))
	}
	
	public func jsonData() -> Data {
		deviceInfoToJsonBytes(deviceInfo: self)
	}
}
