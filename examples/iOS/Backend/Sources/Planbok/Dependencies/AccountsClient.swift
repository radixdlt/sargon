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

	public var getAccounts: GetAccounts
	public var accountsStream: AccountsStream
	public var createAndSaveAccount: CreateAndSaveAccount
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
			}
		)
	}
}
