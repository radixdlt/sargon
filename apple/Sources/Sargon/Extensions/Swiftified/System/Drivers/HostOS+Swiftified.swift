import Foundation

// MARK: - HostOs + SargonModel
extension HostOs: SargonModel {}

extension HostOs {
	public static func ios(version: String) -> HostOs {
		newHostOsIos(version: version)
	}

	public func name() -> String {
		hostOsGetName(hostOs: self)
	}

	public func version() -> String {
		hostOsGetVersion(hostOs: self)
	}

	public func vendor() -> String {
		hostOsGetVendor(hostOs: self)
	}
}
