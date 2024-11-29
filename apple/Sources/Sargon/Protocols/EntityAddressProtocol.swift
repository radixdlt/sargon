import Foundation
import SargonUniFFI

// MARK: - BaseEntityAddressProtocol
/// Just a marker protocol
public protocol BaseEntityAddressProtocol: AddressProtocol {}

// MARK: - EntityAddressProtocol
public protocol EntityAddressProtocol: BaseEntityAddressProtocol {
	init(publicKey: PublicKey, networkID: NetworkID)
}
