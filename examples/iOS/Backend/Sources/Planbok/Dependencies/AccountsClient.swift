//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import Sargon
import DependenciesMacros
import IdentifiedCollections

public typealias Accounts = IdentifiedArrayOf<Account>
extension Array where Element: Identifiable {
	func asIdentified() -> IdentifiedArrayOf<Element> {
		IdentifiedArrayOf.init(uncheckedUniqueElements: self)
	}
}

/// The purpose of this client is to provide WRITE / UPDATE methods of Profile
/// relating to Account(s). READING should be done with `@SharedReader(.accountsForDisplay)`
/// Shared state!
@DependencyClient
public struct AccountsClient: Sendable {
	public typealias AccountByAddress = @Sendable (AccountAddress) throws -> Account
	public typealias CreateAndSaveAccount = @Sendable (DisplayName) async throws -> Account
	public typealias UpdateAccount = @Sendable (Account) async throws -> Void
	public typealias BatchCreateManySavedAccounts = @Sendable (_ count: UInt16) async throws -> Void
	
	public var accountByAddress: AccountByAddress
	public var createAndSaveAccount: CreateAndSaveAccount
	public var updateAccount: UpdateAccount
	public var batchCreateManySavedAccounts: BatchCreateManySavedAccounts
}

extension AccountsClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {

		return Self(
			accountByAddress: { address in
				try os.accountByAddress(address: address)
			},
			createAndSaveAccount: {
				try await os.createAndSaveNewAccount(networkId: os.currentNetworkID, name: $0)
			},
			updateAccount: { account in
				log.debug("Updating account")
				try await os.updateAccount(updated: account)
			},
			batchCreateManySavedAccounts: { count in
				try await os.batchCreateManyAccountsThenSaveOnce(
					count: count,
					networkId: os.currentNetworkID,
					namePrefix: "Unnamed"
				)
			}
		)
	}
}
