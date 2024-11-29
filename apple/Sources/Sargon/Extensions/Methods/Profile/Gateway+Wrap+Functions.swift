import Foundation
import SargonUniFFI

extension Gateway {
	public func getID() -> URL {
		gatewayId(gateway: self)
	}

	public func toString() -> String {
		gatewayToString(gateway: self)
	}

	public var isWellknown: Bool {
		gatewayIsWellknown(gateway: self)
	}

	public static let wellknown: [Gateway] = gatewayWellknownGateways()

	public init(url: String, networkID: NetworkID) throws {
		self = try newGatewayWithUrlOnNetwork(url: url, networkId: networkID)
	}

	public static func forNetwork(id networkID: NetworkID) -> Self {
		newGatewayForNetworkId(networkId: networkID)
	}

	public static let mainnet = gatewayMainnet()
	public static let stokenet = gatewayStokenet()
}
