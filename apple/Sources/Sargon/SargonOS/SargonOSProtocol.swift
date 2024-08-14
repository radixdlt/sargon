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
}

// MARK: Forward calls to `os`
extension SargonOSProtocol {
	
	public func createAccount(
		named accountName: DisplayName
	) async throws -> Account {
		try await os.createAccount(named: accountName)
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
	
	@available(*, deprecated, message: "Consider using faster `accountsForDisplayOnCurrentNetwork` and follow up with ")
	public var accountsOnCurrentNetwork: [Account] {
		get throws {
			try os.accountsOnCurrentNetwork()
		}
	}
	
	public var accountsForDisplayOnCurrentNetwork: [AccountForDisplay] {
		get throws {
			try os.accountsForDisplayOnCurrentNetwork()
		}
	}
	
	public func accountByAddress(_ address: AccountAddress) throws -> Account {
		try os.accountByAddress(address: address)
	}

}
