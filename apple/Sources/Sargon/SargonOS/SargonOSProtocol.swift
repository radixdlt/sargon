//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import SargonUniFFI

/// A protocol enabling us to write `TestOS`
public protocol SargonOSProtocol {
	var os: SargonOS { get }
	
	func createAccount(
		named accountName: DisplayName
	) async throws -> Account
	
	func accounts(on network: NetworkID?) -> [Account]
}

// MARK: Forward calls to `os`
extension SargonOSProtocol {
	
	public func createAccount(
		named accountName: DisplayName
	) async throws -> Account {
		try await os.createAccount(named: accountName)
	}
	
	public func accounts(
		on network: NetworkID? = nil
	) -> [Account] {
		os.accounts(on: network)
	}
}

// MARK: Extensions
extension SargonOSProtocol {
	
	@available(*, deprecated, message: "SHOULD migrate to use more specialized access methods on SargonOS instead, e.g. `accountsOnCurrentNetwork`.")
	public var profile: Profile {
		os.profile()
	}
	
	public var currentNetworkID: NetworkID {
		os.currentNetworkId()
	}
	
	public var gateways: SavedGateways {
		os.gateways()
	}
	
	@available(*, deprecated, message: "Consider using faster `accountsForDisplayOnCurrentNetwork` and follow up with ")
	public var accountsOnCurrentNetwork: [Account] {
		os.accountsOnCurrentNetwork()
	}
	
	public var accountsForDisplayOnCurrentNetwork: [AccountForDisplay] {
		os.accountsForDisplayOnCurrentNetwork()
	}
	
	public func accountByAddress(_ address: AccountAddress) throws -> Account {
		try os.accountByAddress(address: address)
	}

}
