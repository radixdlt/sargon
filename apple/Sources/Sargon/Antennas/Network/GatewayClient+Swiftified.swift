import Foundation
import SargonUniFFI

extension GatewayClient {
	public convenience init(networkID: NetworkID) {
		self.init(networkAntenna: URLSession.shared, networkId: networkID)
	}
}
