import Foundation
import SargonUniFFI

extension GatewayClient {
	public convenience init(networkID: NetworkID) {
		let gateway: Gateway = {
			switch networkID {
			case .stokenet: .stokenet
			case .mainnet: .mainnet
			default: fatalError("support network: \(networkID)")
			}
		}()
		self.init(networkAntenna: URLSession.shared, gateway: gateway)
	}
}
