//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import Sargon
import DependenciesMacros

@DependencyClient
public struct AccountsClient: Sendable {
	public typealias GetAccounts = @Sendable () -> Accounts
	public typealias AccountsStream = @Sendable () -> AsyncStream<Accounts>
	public typealias CreateAndSaveAccount = @Sendable (NetworkID, DisplayName) async throws -> Account
	public typealias UpdateAccount = @Sendable (Account) async throws -> Void
	public typealias BatchCreateManySavedAccounts = @Sendable (_ count: UInt16, _ networkID: NetworkID) async throws -> Void
	
	public var getAccounts: GetAccounts
	public var accountsStream: AccountsStream
	public var createAndSaveAccount: CreateAndSaveAccount
	public var updateAccount: UpdateAccount
	public var batchCreateManySavedAccounts: BatchCreateManySavedAccounts
}

extension AccountsClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		
		let getAccounts: GetAccounts = {
			os.accounts()
		}
		
		return Self(
			getAccounts: getAccounts,
			accountsStream: {
				AsyncStream<Accounts> { continuation in
					Task {
						for await _ in await EventBus.shared.notifications() {
							continuation.yield(getAccounts())
						}
					}
				}
			},
			createAndSaveAccount: {
				try await os.createAndSaveNewAccount(networkId: $0, name: $1)
			},
			updateAccount: { account in
				log.debug("Updating account")
				try await os.updateAccount(updated: account)
			},
			batchCreateManySavedAccounts: {
				count,
				networkID in
				try await os.batchCreateManyAccountsThenSaveOnce(
					count: count,
					networkId: networkID,
					namePrefix: "Unnamed"
				)
			}
		)
	}
}
