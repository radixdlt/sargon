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
	
	public var profile: Profile {
		os.profile()
	}
	
	public var currentNetworkID: NetworkID {
		profile.currentNetworkID
	}
	
	public var accountsOnCurrentNetwork: [Account] {
		accounts(on: currentNetworkID)
	}
}
