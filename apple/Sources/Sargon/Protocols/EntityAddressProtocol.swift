import Foundation
import SargonUniFFI

/// Just a marker protocol
public protocol BaseEntityAddressProtocol: AddressProtocol {}

public protocol EntityAddressProtocol: BaseEntityAddressProtocol {
	init(publicKey: PublicKey, networkID: NetworkID)
}
