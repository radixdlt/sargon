import Foundation
import SargonUniFFI

public protocol EntityAddressProtocol: AddressProtocol {
	init(publicKey: PublicKey, networkID: NetworkID)
}
