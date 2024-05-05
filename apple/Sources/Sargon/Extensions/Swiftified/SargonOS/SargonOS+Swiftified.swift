//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI

public protocol SargonOSProtocol {
	var os: SargonOS { get }
	func createAccount(named accountName: DisplayName) async throws -> Account
	func accounts(on network: NetworkID?) -> Accounts
}
// MARK: Forward calls to `os`
extension SargonOSProtocol {
	func createAccount(named accountName: DisplayName) async throws -> Account {
		try await os.createAccount(named: accountName)
	}
	func accounts(on network: NetworkID?) -> Accounts {
		os.accounts(on: network)
	}
}
extension SargonOS: SargonOSProtocol {
	public var os: SargonOS { self }
}

public typealias SargonOS = SargonOs

extension SargonOS: @unchecked Sendable {}


extension SargonOS {
	
	public var profile: Profile {
		profile()
	}
	
	public var accountsOnCurrentNetwork: Accounts {
		accounts(on: currentNetworkID)
	}
	
	public var currentNetworkID: NetworkID {
		profile().currentNetworkID
	}
	
	public func createAccount(named accountName: DisplayName) async throws -> Account {
		try await createAndSaveNewAccount(networkId: currentNetworkID, name: accountName)
	}
	
	public func accounts(on network: NetworkID? = nil) -> Accounts {
		profile().accounts(on: network)
	}
	
}
