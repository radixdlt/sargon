import Foundation
import SargonUniFFI

// MARK: - SargonOSProtocol
/// A protocol enabling us to write `TestOS`
public protocol SargonOSProtocol {
	var os: SargonOS { get }

	func createAccountWithBDFS(
		networkId: NetworkID?,
		name: DisplayName
	) async throws -> Account
}

// MARK: Forward calls to `os`
extension SargonOSProtocol {
	public func createAccountWithBDFS(
		networkId: NetworkID?,
		name: DisplayName
	) async throws -> Account {
		try await os.createAccountWithBDFS(networkId: networkId, name: name)
	}
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

//	@available(*, deprecated, message: "Consider using faster `accountsForDisplayOnCurrentNetwork` and follow up with ")
//	public var accountsOnCurrentNetwork: [Account] {
//		get throws {
//			try os.accountsOnCurrentNetwork()
//		}
//	}
//
//	public var accountsForDisplayOnCurrentNetwork: [AccountForDisplay] {
//		get throws {
//			try os.accountsForDisplayOnCurrentNetwork()
//		}
//	}
//
//	public func accountByAddress(_ address: AccountAddress) throws -> Account {
//		try os.accountByAddress(address: address)
//	}
}
