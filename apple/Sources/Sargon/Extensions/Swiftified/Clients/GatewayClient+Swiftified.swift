import Foundation
import SargonUniFFI

extension GatewayClient {
	public convenience init(networkID: NetworkID) {
		self.init(networkingDriver: URLSession.shared, networkId: networkID)
	}
}
