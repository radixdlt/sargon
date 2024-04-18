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
	
	public init(jsonString: String) throws {
		self = try newDeviceInfoFromJsonString(jsonString: jsonString)
	}
	
	public func jsonString() -> String {
		deviceInfoToJsonString(deviceInfo: self)
	}
}
