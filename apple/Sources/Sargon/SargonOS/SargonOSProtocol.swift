import Foundation
import SargonUniFFI

// MARK: - SargonOSProtocol
/// A protocol enabling us to write `TestOS`
public protocol SargonOSProtocol {
	var os: SargonOS { get }
}

// MARK: Extensions
extension SargonOSProtocol {
	public var currentNetworkID: NetworkID {
		get throws {
			try os.currentNetworkId()
		}
	}

	public var gateways: SavedGateways {
		get throws {
			try os.gateways()
		}
	}
}
