import Foundation
import SargonUniFFI

extension DeviceInfo {
	public init(jsonData: some DataProtocol) throws {
		self = try newDeviceInfoFromJsonBytes(jsonBytes: Data(jsonData))
	}

	public func jsonData() -> Data {
		deviceInfoToJsonBytes(deviceInfo: self)
	}
}
